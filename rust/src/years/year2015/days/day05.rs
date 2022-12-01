use std::fs::File;
use std::io::{BufReader, BufRead};
use itertools::Itertools;

fn is_vowel(c: char) -> bool {
    c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
}

fn vowel_count(s: String) -> i32 {
    s.chars().filter(|c| is_vowel(*c)).count() as i32
}

pub fn a() -> i64 {
    let f = BufReader::new(File::open("input/2015/day05.txt").unwrap());
    
    let mut nice_names = 0;

    for line in f.lines() {
        let name = line.unwrap();
        let mut double = false;

        if name.contains("ab") || name.contains("cd") || name.contains("pq") || name.contains("xy") {
            continue;
        }

        for (c, next) in name.chars().tuple_windows() {
            if c == next {
                double = true;
                break;
            }
        }

        if double && vowel_count(name) >= 3 {
            nice_names += 1;
        }
    }

    nice_names
}

pub fn b() -> i64 {
    let f = BufReader::new(File::open("input/2015/day05.txt").unwrap());
    
    let mut nice_names = 0;

    for line in f.lines() {
        let name = line.unwrap();
        let mut double = false;
        let mut xyx = false;

        for (i, (c, d)) in name.chars().tuple_windows().enumerate() {
            let pair = format!("{}{}", c, d);
            if name.rfind(&pair[..]).unwrap() > i + 1 {
                double = true;
            }
        }

        for (c, d, e) in name.chars().tuple_windows() {
            if c == e && c != d {
                xyx = true;
            }
        }

        if double && xyx {
            nice_names += 1;
        }
    }

    nice_names
}