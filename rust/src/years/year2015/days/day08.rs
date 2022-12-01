use std::{io::{BufReader, BufRead}, fs::File};

pub fn a() -> i64 {
    let f = BufReader::new(File::open("input/2015/day08.txt").unwrap());
    let mut count = 0;

    for line in f.lines() {
        let line = line.unwrap();
        let line = &line[1..line.len()-1];

        let mut memory_count = 0;

        let mut it = line.chars();
        while let Some(c) = it.next() {
            memory_count += 1;
            if c == '\\' {
                if let Some(c) = it.next() {
                    if c == 'x' {
                        it.next();
                        it.next();
                    }
                }
            }
        }

        count += line.len() as i64 + 2 - memory_count;
    }

    count
}

pub fn b() -> i64 {
    let f = BufReader::new(File::open("input/2015/day08.txt").unwrap());
    let mut count = 0;

    for line in f.lines() {
        let line = line.unwrap();
        let line = &line[1..line.len()-1];
        let mut str_count = 4;
        for c in line.chars() {
            if c == '\\' || c == '\'' || c == '"' {
                str_count += 1;
            }
        }

        count += str_count;
    }

    count
}