#![allow(dead_code)]

use std::io::{BufReader, BufRead};
use std::fs::File;

pub fn a() {
    let f = BufReader::new(File::open("input/day01.txt").unwrap());
    let mut count = 0;
    let iter = f.lines();
    let mut last = 0;
    
    for line in iter {
        let next = line.unwrap().parse().unwrap();
        if last != 0 && next > last {
            count += 1;
        }
        last = next;
    }

    println!("{}", count);
}

pub fn b() {
    let f = BufReader::new(File::open("input/day01.txt").unwrap());
    let mut nums: Vec<i64> = Vec::new();
    for line in f.lines() {
        let num = line.unwrap().parse().unwrap();
        nums.push(num);
    }

    let mut count = 0;
    let mut last = nums[0] + nums[1] + nums[2];
    for i in 1..nums.len()-2 {
        let sum = nums[i] + nums[i+1] + nums[i+2];
        if sum > last {
            count += 1;
        }
        last = sum;
    }

    println!("{}", count);
}