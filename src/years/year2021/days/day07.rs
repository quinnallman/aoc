use std::fs;

pub fn run() -> (i64, i64) {
    (a(), b())
}

fn a() -> i64 {
    let positions: Vec<i64> = fs::read_to_string("input/2021/day07.txt").unwrap().split(',').map(|x| x.parse::<i64>().unwrap()).collect();
    let max = positions.iter().max().unwrap();

    let mut min = i64::MAX;
    for i in 0..max+1 {
        let diff = positions.iter().map(|x| (x - i).abs()*((x - i).abs() + 1)/2).reduce(|a, b| a+b).unwrap();
        if diff < min {
            min = diff;
        }
    }

    min
}

fn b() -> i64 {
    0
}