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
                AST::Garbage(_) => (),
                ast => panic!("Expected Garbage; received {:?}", ast)
            }
        }
    }

    #[test]
    fn recognizes_groups() {
        for t in GROUP_TESTS {
            match parse(t.source).unwrap() {
                AST::Group(_) => (),
                ast => panic!("Expected Group; received {:?}", ast)
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
            let ast = parse(t.source).unwrap();
            assert_eq!(ast.count_groups(), t.expect, "in {}", t.source);
        }
    }

    #[test]
    fn scores() {
        for t in SCORE_TESTS {
            let ast = parse(t.source).unwrap();
            assert_eq!(ast.score(), t.expect, "in {} {:?}", t.source, ast);
        }
    }

    #[test]
    fn garbage_len() {
        for t in GARBAGE_TESTS {
            let ast = parse(t.source).unwrap();
            assert_eq!(ast.garbage_len(), t.expect, "in {} {:?}", t.source, ast);
        }
    }

    struct TestCase<T, U> { source: T, expect: U}

    macro_rules! define_test_data {
        (const $name:ident = < $et:tt > { $($e:expr => $v:expr),+ $(,)* }) => {
            const $name: &'static [TestCase<&'static str, $et>] = &[
                $(TestCase { source: $e, expect: $v }),*
            ];
        };
    }

    define_test_data! {
        const GARBAGE_TESTS = <usize>{
            "<>" => 0,
            "<random characters>" => 17,
            "<<<<>" => 3,
            "<{!>}>" => 2,
            "<!!>" => 0,
            "<!!!>>" => 0,
            "<{o\"i!a,<{i<a>" => 10,
        }
    }

    define_test_data! {
        const GROUP_TESTS = <usize>{
            "{}" => 1,
            "{{{}}}" => 3,
            "{{},{}}" => 3,
            "{{{},{},{{}}}}" => 6,
            "{<{},{},{{}}>}" => 1,
            "{<a>,<a>,<a>,<a>}" => 1,
            "{{<a>},{<a>},{<a>},{<a>}}" => 5,
            "{{<!>},{<!>},{<!>},{<a>}}" => 2,
        }
    }

    define_test_data! {
        const SCORE_TESTS = <usize>{
            "{}" => 1,
            "{{{}}}" => /* 1 + 2 + 3 = */ 6,
            "{{},{}}" => /* 1 + 2 + 2 = */ 5,
            "{{{},{},{{}}}}" => /* 1 + 2 + 3 + 3 + 3 + 4 = */ 16,
            "{<a>,<a>,<a>,<a>}" => 1,
            "{{<ab>},{<ab>},{<ab>},{<ab>}}" => /* 1 + 2 + 2 + 2 + 2 = */ 9,
            "{{<!!>},{<!!>},{<!!>},{<!!>}}" => /* 1 + 2 + 2 + 2 + 2 = */ 9,
            "{{<a!>},{<a!>},{<a!>},{<ab>}}" => /* 1 + 2 = */ 3,
        }
    }
}
