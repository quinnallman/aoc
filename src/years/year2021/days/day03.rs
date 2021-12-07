use std::io::{BufReader, BufRead};
use std::fs::File;

pub fn run() -> (i64, i64) {
    (a(), b())
}

fn count_binary_digits(strs: &[String]) -> ([i64; 12], [i64; 12]) {
    let mut count_0 = [0; 12];
    let mut count_1 = [0; 12];

    for line in strs {
        for (i, digit) in line.chars().enumerate() {
            if digit == '0' {
                count_0[i] += 1;
            } else if digit == '1' {
                count_1[i] += 1;
            }
        }
    }

    (count_0, count_1)
}

fn a() -> i64 {
    let f = BufReader::new(File::open("input/2021/day03.txt").unwrap());
    let strings: Vec<String> = f.lines().map(|s| s.unwrap()).collect();

    let (count_0, count_1) = count_binary_digits(&strings);

    let mut gamma = 0;
    let mut epsilon = 0;

    for i in 0..12 {
        gamma *= 2;
        epsilon *= 2;

        if count_1[i] > count_0[i] {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }

    gamma * epsilon
}

fn b() -> i64 {
    let f = BufReader::new(File::open("input/2021/day03.txt").unwrap());
    let mut strings: Vec<String> = f.lines().map(|s| s.unwrap()).collect();

    let mut co2_strs = strings.clone();

    let mut index = 0;
    while strings.len() > 1 {
        let (count_0, count_1) = count_binary_digits(&strings);
    
        if count_1[index] >= count_0[index] {
            strings.retain(|x| x.chars().nth(index).unwrap() == '1');
        } else {
            strings.retain(|x| x.chars().nth(index).unwrap() == '0');
        }

        index += 1;
    }

    index = 0;
    while co2_strs.len() > 1 {
        let (count_0, count_1) = count_binary_digits(&co2_strs);
    
        if count_1[index] >= count_0[index] {
            co2_strs.retain(|x| x.chars().nth(index).unwrap() == '0');
        } else {
            co2_strs.retain(|x| x.chars().nth(index).unwrap() == '1');
        }

        index += 1;
    }

    let oxy = i64::from_str_radix(&(strings.pop().unwrap())[..], 2).unwrap();
    let co2 = i64::from_str_radix(&(co2_strs.pop().unwrap())[..], 2).unwrap();

    oxy * co2
}