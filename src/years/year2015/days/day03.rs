use std::{fs, collections::HashMap};

pub fn a() -> i64 {
    let f = fs::read_to_string("input/2015/day03.txt").unwrap();

    let mut visit: HashMap<(i64, i64), i64> = HashMap::new();
    let mut pos = (0, 0);
    visit.insert(pos, 1);

    for c in f.chars() {
        match c {
            '^' => {
                pos.1 -= 1;
            },
            'v' => {
                pos.1 += 1;
            },
            '>' => {
                pos.0 += 1;
            },
            '<' => {
                pos.0 -= 1;
            },
            _ => {},
        }

        let count = visit.entry(pos).or_insert(1);
        *count += 1;
    }

    visit.len() as i64
}

pub fn b() -> i64 {
    let f = fs::read_to_string("input/2015/day03.txt").unwrap();

    let mut visit: HashMap<(i64, i64), i64> = HashMap::new();
    let mut pos = (0, 0);
    let mut robo_pos = (0, 0);
    let mut robo_turn = false;
    visit.insert(pos, 2);

    for c in f.chars() {
        let mut who = &mut pos;
        if robo_turn {
            who = &mut robo_pos;
        }

        match c {
            '^' => {
                who.1 -= 1;
            },
            'v' => {
                who.1 += 1;
            },
            '>' => {
                who.0 += 1;
            },
            '<' => {
                who.0 -= 1;
            },
            _ => {},
        }

        if visit.contains_key(who) {
            visit.insert(*who, visit.get(who).unwrap() + 1);
        } else {
            visit.insert(*who, 1);
        }

        robo_turn = !robo_turn;
    }

    visit.len() as i64
}