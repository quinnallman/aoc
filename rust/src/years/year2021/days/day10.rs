use std::{io::{BufReader, BufRead}, fs::File};

use itertools::Itertools;

pub fn a() -> i64 {
    let f = BufReader::new(File::open("input/2021/day10.txt").unwrap());
    let mut sum = 0;
    for line in f.lines() {
        let line = line.unwrap();
        let mut stack = Vec::new();
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                _ => {
                    if c != matcher(stack.pop().unwrap()) {
                        sum += corrupt(c);
                        break;
                    }
                },
            }
        }
    }

    sum
}

fn matcher(c: char) -> char {
    match c {
        '(' => ')',
        ')' => '(',
        '[' => ']',
        ']' => '[',
        '<' => '>',
        '>' => '<',
        '{' => '}',
        '}' => '{',
        _ => '!',
    }
}

fn missing(c: char) -> i64 {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => 0,
    }
}

fn corrupt(c: char) -> i64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

pub fn b() -> i64 {
    let f = BufReader::new(File::open("input/2021/day10.txt").unwrap());
    let mut scores = Vec::new();
    'lines: for line in f.lines() {
        let line = line.unwrap();
        let mut stack = Vec::new();

        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                _ => {
                    if matcher(c) != stack.pop().unwrap() {
                        continue 'lines;
                    }
                },
            }
        }

        scores.push(
            stack
                .iter()
                .rev()
                .fold(0, |acc, s| acc * 5 + missing(*s))
        );
    }

    let i_middle = scores.len() / 2;
    scores.into_iter().sorted().nth(i_middle).unwrap()
}