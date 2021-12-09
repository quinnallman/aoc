use std::{io::{BufReader, BufRead}, fs::File, str::FromStr, collections::HashMap};
use itertools::Itertools;

#[derive(Debug)]
struct Entry {
    city1: String,
    city2: String,
    distance: i64,
}

impl FromStr for Entry {
    type Err = std::num::ParseIntError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let p1 = line.find(" to ").unwrap();
        let p2 = line.find(" = ").unwrap();
        Ok(Self {
            city1: String::from(&line[..p1]),
            city2: String::from(&line[p1+4..p2]),
            distance: line[p2+3..].parse::<i64>()?,
        })
    }
}

fn cost(cities: Vec<usize>, adj: &[[i64; 10]; 10]) -> i64 {
    let mut sum = 0;
    for (&c1, &c2) in cities.iter().tuple_windows() {
        sum += adj[c1][c2];
    }
    sum
}

fn rtsp(curr: Vec<usize>, left: Vec<usize>, adj: &[[i64; 10]; 10]) -> i64 {
    if left.is_empty() {
        return cost(curr, adj);
    }

    let mut min = i64::MAX;
    for (i, &city) in left.iter().enumerate() {
        let mut left2 = left.clone();
        let mut curr2 = curr.clone();
        left2.remove(i);
        curr2.push(city);
        let cost = rtsp(curr2, left2, adj);
        if cost < min {
            min = cost;
        }
    }

    min
}

fn rtsp_max(curr: Vec<usize>, left: Vec<usize>, adj: &[[i64; 10]; 10]) -> i64 {
    if left.is_empty() {
        return cost(curr, adj);
    }

    let mut max = 0;
    for (i, &city) in left.iter().enumerate() {
        let mut left2 = left.clone();
        let mut curr2 = curr.clone();
        left2.remove(i);
        curr2.push(city);
        let cost = rtsp_max(curr2, left2, adj);
        if cost > max {
            max = cost;
        }
    }

    max
}

pub fn a() -> i64 {
    let f = BufReader::new(File::open("input/2015/day09.txt").unwrap());

    let mut count = 0;
    let mut map: HashMap<String, usize> = HashMap::new();
    let mut adj_matrix = [[0; 10]; 10];
    for line in f.lines() {
        let entry = line.unwrap().parse::<Entry>().unwrap();
        if !map.contains_key(&entry.city1) {
            map.insert(entry.city1.clone(), count);
            count += 1;
        }
        if !map.contains_key(&entry.city2) {
            map.insert(entry.city2.clone(), count);
            count += 1;
        }

        let i1 = *map.get(&entry.city1).unwrap();
        let i2 = *map.get(&entry.city2).unwrap();

        adj_matrix[i1][i2] = entry.distance;
        adj_matrix[i2][i1] = entry.distance;
    }

    rtsp(Vec::new(), (0..count).collect::<Vec<usize>>(), &adj_matrix)
}

pub fn b() -> i64 {
    let f = BufReader::new(File::open("input/2015/day09.txt").unwrap());

    let mut count = 0;
    let mut map: HashMap<String, usize> = HashMap::new();
    let mut adj_matrix = [[0; 10]; 10];
    for line in f.lines() {
        let entry = line.unwrap().parse::<Entry>().unwrap();
        if !map.contains_key(&entry.city1) {
            map.insert(entry.city1.clone(), count);
            count += 1;
        }
        if !map.contains_key(&entry.city2) {
            map.insert(entry.city2.clone(), count);
            count += 1;
        }

        let i1 = *map.get(&entry.city1).unwrap();
        let i2 = *map.get(&entry.city2).unwrap();

        adj_matrix[i1][i2] = entry.distance;
        adj_matrix[i2][i1] = entry.distance;
    }

    rtsp_max(Vec::new(), (0..count).collect::<Vec<usize>>(), &adj_matrix)
}