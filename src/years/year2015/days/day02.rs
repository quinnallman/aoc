use std::io::{BufReader, BufRead};
use std::fs::File;

pub fn run() -> (i64, i64) {
    let f = BufReader::new(File::open("input/2015/day02.txt").unwrap());
    
    let mut paper_area = 0;
    let mut ribbon_area = 0;

    for line in f.lines() {
        let line = line.unwrap_or_else(|_| String::from("forward 0"));
        let tokens: Vec<&str> = line.split('x').collect();

        let dimens = (tokens[0].parse::<i64>().unwrap(), tokens[1].parse::<i64>().unwrap(), tokens[2].parse::<i64>().unwrap());
        paper_area += area_req(dimens.0, dimens.1, dimens.2);
        ribbon_area += ribbon_req(dimens.0, dimens.1, dimens.2);
    }

    (paper_area, ribbon_area)
}

fn area_req(l: i64, w: i64, h: i64) -> i64 {
    let min_side = std::cmp::min(std::cmp::min(l*w, w*h), h*l);
    2*l*w + 2*w*h + 2*h*l + min_side
}

fn ribbon_req(l: i64, w: i64, h: i64) -> i64 {
    let perim = l + w + h - std::cmp::max(std::cmp::max(l, w), h);
    2 * perim + l*w*h
}