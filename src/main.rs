// http://adventofcode.com/2017/day/9

// This implementation uses a recursive-descent parser. This particular problem
// can be more easily solved using regular expressions (see my Python solution)
// or a state machine and a couple of accumulators. This strategy meets my
// learning objectives better than those.

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let filename = "data/input-9.txt";
    let mut f = File::open(filename).expect("file not found");
    let mut source = String::new();
    f.read_to_string(&mut source).expect("can't read the file");
    let ast = parse(&source);
    println!("Part 1: {}", score(&ast));
    println!("Part 2: {}", garbage_len(&ast));
}

#[derive(Debug)]
enum AST {
    Group(Vec<AST>),
    Garbage(String),
}

fn garbage(iter: &mut std::str::Chars) -> AST {
    let mut string = String::new();
    while let Some(c) = iter.next() {
        match c {
            '!' => match iter.next() {
                Some(_) => (),
                None => panic!("unexpected end of string after '!'")
            },
            '>' => return AST::Garbage(string),
            _ => string.push(c)
        }
    }
    panic!("invalid garbage string: unterminated '<'")
}

fn group(iter: &mut std::str::Chars) -> AST {
    let mut children = Vec::new();
    while let Some(c) = iter.next() {
        match c {
            // TODO DRY w/ parse
            '<' => children.push(garbage(iter)),
            '{' => children.push(group(iter)),
            '}' => break,
            _ => ()
        }
    }
    return AST::Group(children)
}

fn parse(src: &str) -> AST {
    let mut iter = src.chars();
    return match iter.next() {
        Some('<') => garbage(&mut iter),
        Some('{') => group(&mut iter),
        Some(c) => panic!("invalid string: expected '<' or '{{', found {:?}", c),
        None => panic!("invalid string: expected non-empty string")
    }
}

#[allow(dead_code)]
fn count_groups(node: &AST) -> usize {
    match node {
        &AST::Group(ref children) => 1 + children.iter().map(count_groups).sum::<usize>(),
        _ => 0
    }
}

fn score(node: &AST) -> usize {
    fn depths(node: &AST, i: usize) -> usize {
        match node {
            &AST::Group(ref children) => i + children.iter().map(|n| depths(n, i+1)).sum::<usize>(),
            _ => 0
        }
    }
    return depths(node, 1);
}

fn garbage_len(node: &AST) -> usize {
    match node {
        &AST::Garbage(ref s) => s.len(),
        &AST::Group(ref children) => children.iter().map(garbage_len).sum::<usize>(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // TODO test cases for error conditions
    // TODO learn macros https://github.com/BurntSushi/fst/blob/master/src/raw/tests.rs#L111-L158

    #[test]
    fn recognizes_garbage() {
        for t in GARBAGE_TESTS {
            match parse(t.source) {
                AST::Garbage(_)=> (),
                node => panic!("Expected Garbage; received {:?}", node)
            }
        }
    }

    #[test]
    fn recognizes_groups() {
        for t in GROUP_TESTS {
            match parse(t.source) {
                AST::Group(_) => (),
                node => panic!("Expected Group; received {:?}", node)
            }
        }
    }

    #[test]
    fn counts_groups() {
        for t in GROUP_TESTS {
            let node = parse(t.source);
            assert_eq!(count_groups(&node), t.count, "in {}", t.source);
        }
    }

    #[test]
    fn computes_scores() {
        for t in SCORE_TESTS {
            let node = parse(t.source);
            assert_eq!(score(&node), t.score, "in {} {:?}", t.source, node);
        }
    }

    #[test]
    fn computes_garbage_len() {
        for t in GARBAGE_TESTS {
            let node = parse(t.source);
            assert_eq!(garbage_len(&node), t.length, "in {} {:?}", t.source, node);
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
        ScoreTest { source: "{{{},{},{{}}", score: /* 1 + 2 + 3 + 3 + 3 + 4 = */ 16 },
        ScoreTest { source: "{<a>,<a>,<a>,<a>}", score: 1 },
        ScoreTest { source: "{{<ab>},{<ab>},{<ab>},{<ab>}}", score: /* 1 + 2 + 2 + 2 + 2 = */ 9 },
        ScoreTest { source: "{{<!!>},{<!!>},{<!!>},{<!!>}}", score: /* 1 + 2 + 2 + 2 + 2 = */ 9 },
        ScoreTest { source: "{{<a!>},{<a!>},{<a!>},{<ab>}}", score: /* 1 + 2 = */ 3 },
    ];}
