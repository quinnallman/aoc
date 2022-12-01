use std::io::{BufReader, BufRead};
use std::fs::File;

pub fn a() -> i64 {
    let f = BufReader::new(File::open("input/2021/day02.txt").unwrap());
    let mut position = (0, 0);
    
    for line in f.lines() {
        let line = line.unwrap_or_else(|_| String::from("forward 0"));
        let tokens: Vec<&str> = line.split_ascii_whitespace().collect();

        let direction = tokens[0];
        let distance = tokens[1].parse::<i64>().unwrap_or(0);
        match direction {
            "forward" => {
                position.0 += distance;
            },
            "up" => {
                position.1 -= distance;
            },
            "down" => {
                position.1 += distance;
            },
            _ => {
                continue;
            },
        }
    }

    position.0 * position.1
}

pub fn b() -> i64 {
    let f = BufReader::new(File::open("input/2021/day02.txt").unwrap());
    let mut position = (0, 0, 0);
    
    for line in f.lines() {
        let line = line.unwrap_or_else(|_| String::from("forward 0"));
        let tokens: Vec<&str> = line.split_ascii_whitespace().collect();

        let direction = tokens[0];
        let distance = tokens[1].parse::<i64>().unwrap_or(0);
        match direction {
            "forward" => {
                position.0 += distance;
                position.2 += distance * position.1;
            },
            "up" => {
                position.1 -= distance;
            },
            "down" => {
                position.1 += distance;
            },
            _ => {
                break;
            },
        }
    }

    position.0 * position.2
}