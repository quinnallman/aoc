use std::{io::{BufReader, BufRead}, fs::File, str::FromStr};

#[derive(Debug, Clone)]
enum SnailNumber {
    Number(i64),
    Pair(SnailPair),
}

#[derive(Debug, Clone)]
struct SnailPair {
    pub left: Box<SnailNumber>,
    pub right: Box<SnailNumber>,
}

impl SnailPair {
    fn add(self: &Self, other: &SnailPair) -> SnailPair {
        SnailPair {
            left: Box::new(SnailNumber::Pair(self.clone())),
            right: Box::new(SnailNumber::Pair(other.clone())),
        }
    }

    fn explode(self: &Self, depth: i64) {
        if depth == 3 {
            
        }
    }
}

impl FromStr for SnailPair {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = &s[1..s.len()-1];
        let mut depth = 0;
        let mut comma_index = 0;
        for c in s.chars() {
            match c {
                '[' => {
                    depth += 1;
                },
                ']' => {
                    depth -= 1;
                },
                ',' => {
                    if depth == 0 {
                        break;
                    }
                },
                _ => {},
            }

            comma_index += 1;
        }

        let c = s.chars().next().unwrap();
        let left = match c {
            '[' => {
                Box::new(SnailNumber::Pair(s[..comma_index].parse::<SnailPair>().unwrap()))
            },
            _ => {
                Box::new(SnailNumber::Number(c as i64 - '0' as i64))
            },
        };

        let c = s.chars().nth(comma_index+1).unwrap();
        let right = match c {
            '[' => {
                Box::new(SnailNumber::Pair(s[comma_index+1..].parse::<SnailPair>().unwrap()))
            },
            _ => {
                Box::new(SnailNumber::Number(c as i64 - '0' as i64))
            },
        };

        Ok(SnailPair {
            left,
            right,
        })
    }
}

pub fn a() -> i64 {
    let mut numbers = Vec::new();
    let f = BufReader::new(File::open("input/2021/day18.txt").unwrap());
    for line in f.lines() {
        let line = line.unwrap();
        let sp = line.parse::<SnailPair>().unwrap();
        println!("{:?}", sp);
        numbers.push(sp);
    }

    let result = numbers[0].add(&numbers[1]);
    println!("{:?}", result);

    0
}

pub fn b() -> i64 {
    0
}