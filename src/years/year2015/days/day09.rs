use std::{io::{BufReader, BufRead}, fs::File};

pub fn run() -> (i64, i64) {
    (a(), b())
}

fn a() -> i64 {
    let f = BufReader::new(File::open("input/2015/day08.txt").unwrap());

    for line in f.lines() {
        println!("{}", line.unwrap());
    }

    0
}

fn b() -> i64 {
    0
}