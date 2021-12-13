use std::{cmp::Ordering, io::{BufReader, BufRead}, fs::File};

fn count(containers: &mut Vec<i64>, target: i64) -> i64 {
    if target == 0 {
        return 1;
    }

    let mut num = 0;
    while !containers.is_empty() {
        let c = containers.pop().unwrap();
        num += count(&mut containers.clone(), target - c);
    }

    num
}

pub fn a() -> i64 {
    let f = BufReader::new(File::open("input/2015/day17.txt").unwrap());
    let mut containers = Vec::new();
    for line in f.lines() {
        let line = line.unwrap();
        containers.push(line.parse::<i64>().unwrap());
    }

    count(&mut containers, 150)
}


fn min_containers(curr: &mut Vec<i64>, containers: &mut Vec<i64>, target: i64) -> (i64, i64) {
    match target.cmp(&0) {
        Ordering::Less => { return (0, 0); },
        Ordering::Equal => { return (curr.len() as i64, 1); }
        _ => {},
    }

    let mut min = (i64::MAX, 0);
    while !containers.is_empty() {
        let c = containers.pop().unwrap();
        curr.push(c);

        let x = min_containers(curr, &mut containers.clone(), target - c);
        if x.0 < min.0 && x.1 > 0 {
            min = x;
        } else if x.0 == min.0 {
            min.1 += x.1
        }

        curr.pop();
    }

    min
}

pub fn b() -> i64 {
    let f = BufReader::new(File::open("input/2015/day17.txt").unwrap());
    let mut containers = Vec::new();
    for line in f.lines() {
        let line = line.unwrap();
        containers.push(line.parse::<i64>().unwrap());
    }

    let x = min_containers(&mut Vec::new(), &mut containers, 150);
    x.1
}