use std::fs;

pub fn run() -> (i64, i64) {
    (a(), b())
}

fn a() -> i64 {
    let f = fs::read_to_string("input/2015/day01.txt").unwrap();

    let mut floor = 0;
    for char in f.chars() {
        match char {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => continue,
        }
    }

    floor
}

fn b() -> i64 {
    let f = fs::read_to_string("input/2015/day01.txt").unwrap();

    let mut floor = 0;
    let mut basement_instruction = 0;
    for (i, char) in f.chars().enumerate() {
        match char {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => continue,
        }
        if basement_instruction == 0 && floor < 0 {
            basement_instruction = i as i64;
        }
    }

    basement_instruction + 1
}