// http://adventofcode.com/2017/day/9

// This implementation uses a recursive-descent parser. This particular problem
// can be more easily solved using regular expressions (see my Python solution)
// or a state machine and a couple of accumulators. This strategy meets my
// learning objectives better than those.

use std::fmt;
use std::fs::File;
use std::result::Result;
use std::io::prelude::*;

fn main() {
    let filename = "data/input-9.txt";
    let mut f = File::open(filename).expect("file not found");
    let mut source = String::new();
    f.read_to_string(&mut source).expect("can't read the file");
    let result = parse(&source);
    if let Err(err) = result {
        println!("syntax error: {}", err);
        return;
    }
    let ast = result.unwrap();
    println!("Part 1: {}", ast.score());
    println!("Part 2: {}", ast.garbage_len());
}

#[derive(Debug)]
enum AST {
    Group(Vec<AST>),
    Garbage(String),
}

impl AST {
    #[allow(dead_code)]
    fn count_groups(&self) -> usize {
        match self {
            &AST::Group(ref children) => 1 + children.iter().map(AST::count_groups).sum::<usize>(),
            _ => 0
        }
    }

    fn score(&self) -> usize {
        fn depths(node: &AST, i: usize) -> usize {
            match node {
                &AST::Group(ref children) => i + children.iter().map(|n| depths(n, i+1)).sum::<usize>(),
                _ => 0
            }
        }
        return depths(self, 1);
    }

    fn garbage_len(&self) -> usize {
        match self {
            &AST::Garbage(ref s) => s.len(),
            &AST::Group(ref children) => children.iter().map(AST::garbage_len).sum::<usize>(),
        }
    }
}

#[derive(Debug)]
struct ParseError {
    message: String
}

impl ParseError {
    fn result<T>(msg: &str) -> Result<T, ParseError> {
        Err(ParseError{message: msg.to_string()})
    }

    fn format<T>(msg: String) -> Result<T, ParseError> {
        Err(ParseError{message: msg})
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

type ParseResult = Result<AST, ParseError>;

fn garbage(iter: &mut std::str::Chars) -> ParseResult {
    let mut string = String::new();
    while let Some(c) = iter.next() {
        match c {
            '!' => match iter.next() {
                Some(_) => (),
                None => return ParseError::result("unexpected end of string after '!'")
            },
            '>' => return Ok(AST::Garbage(string)),
            _ => string.push(c)
        }
    }
    ParseError::result("unterminated '<'")
}

fn group(iter: &mut std::str::Chars) -> ParseResult {
    let mut children = Vec::new();
    while let Some(c) = iter.next() {
        match c {
            // TODO DRY w/ parse
            '<' => children.push(garbage(iter)?),
            '{' => children.push(group(iter)?),
            '}' => return Ok(AST::Group(children)),
            _ => ()
        }
    }
    ParseError::result("unterminated '{'")
}

fn parse(src: &str) -> ParseResult {
    let mut iter = src.chars();
    match iter.next() {
        Some('<') => garbage(&mut iter),
        Some('{') => group(&mut iter),
        Some(c) => ParseError::format(format!("expected '<' or '{{', found {:?}", c)),
        None => ParseError::result("expected non-empty string")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognizes_garbage() {
        for t in GARBAGE_TESTS {
            match parse(t.source).unwrap() {
                AST::Garbage(_)=> (),
                node => panic!("Expected Garbage; received {:?}", node)
            }
        }
    }

    #[test]
    fn recognizes_groups() {
        for t in GROUP_TESTS {
            match parse(t.source).unwrap() {
                AST::Group(_) => (),
                node => panic!("Expected Group; received {:?}", node)
            }
        }
    }

    #[test]
    fn syntax_errors() {
        const SYNTAX_ERROR_TESTS: &'static [&'static str] = &[
            "",
            "x",
            "<",
            "<!",
            "<random characters",
            "<random characters!>",
            "{",
            "{<",
            "{<>",
            "{<!>}",
            "{{}",
        ];
        for s in SYNTAX_ERROR_TESTS {
            let result = parse(s);
            assert!(result.is_err(), "{} -> {:?}", s, result);
        }
    }

    #[test]
    fn count_groups() {
        for t in GROUP_TESTS {
            let node = parse(t.source).unwrap();
            assert_eq!(node.count_groups(), t.count, "in {}", t.source);
        }
    }

    #[test]
    fn scores() {
        for t in SCORE_TESTS {
            let node = parse(t.source).unwrap();
            assert_eq!(node.score(), t.score, "in {} {:?}", t.source, node);
        }
    }

    #[test]
    fn garbage_len() {
        for t in GARBAGE_TESTS {
            let node = parse(t.source).unwrap();
            assert_eq!(node.garbage_len(), t.length, "in {} {:?}", t.source, node);
        }
    }

    struct GarbageTest {
        source: &'static str,
        length: usize,
    }

    const GARBAGE_TESTS: &'static [GarbageTest] = &[
        GarbageTest { source: "<>", length: 0 },
        GarbageTest { source: "<random characters>", length: 17 },
        GarbageTest { source: "<<<<>", length: 3 },
        GarbageTest { source: "<{!>}>", length: 2 },
        GarbageTest { source: "<!!>", length: 0 },
        GarbageTest { source: "<!!!>>", length: 0 },
        GarbageTest { source: "<{o\"i!a,<{i<a>", length: 10 },
    ];

    struct GroupTest {
        source: &'static str,
        count: usize,
    }

    const GROUP_TESTS: &'static [GroupTest] = &[
        GroupTest { source: "{}", count: 1 },
        GroupTest { source: "{{{}}}", count: 3 },
        GroupTest { source: "{{},{}}", count: 3 },
        GroupTest { source: "{{{},{},{{}}}}", count: 6 },
        GroupTest { source: "{<{},{},{{}}>}", count: 1 },
        GroupTest { source: "{<a>,<a>,<a>,<a>}", count: 1 },
        GroupTest { source: "{{<a>},{<a>},{<a>},{<a>}}", count: 5 },
        GroupTest { source: "{{<!>},{<!>},{<!>},{<a>}}", count: 2 },
    ];

    struct ScoreTest {
        source: &'static str,
        score: usize,
    }

    const SCORE_TESTS: &'static [ScoreTest] = &[
        ScoreTest { source: "{}", score: 1 },
        ScoreTest { source: "{{{}}}", score: /* 1 + 2 + 3 = */ 6 },
        ScoreTest { source: "{{},{}}", score: /* 1 + 2 + 2 = */ 5 },
        ScoreTest { source: "{{{},{},{{}}}}", score: /* 1 + 2 + 3 + 3 + 3 + 4 = */ 16 },
        ScoreTest { source: "{<a>,<a>,<a>,<a>}", score: 1 },
        ScoreTest { source: "{{<ab>},{<ab>},{<ab>},{<ab>}}", score: /* 1 + 2 + 2 + 2 + 2 = */ 9 },
        ScoreTest { source: "{{<!!>},{<!!>},{<!!>},{<!!>}}", score: /* 1 + 2 + 2 + 2 + 2 = */ 9 },
        ScoreTest { source: "{{<a!>},{<a!>},{<a!>},{<ab>}}", score: /* 1 + 2 = */ 3 },
    ];}
