use std::{io::{BufReader, BufRead}, fs::File, str::FromStr};

#[derive(Debug)]
struct ParseError(pub String);

#[derive(Debug)]
struct Reindeer {
    pub name: String,
    pub speed: i64,
    pub stamina: i64,
    pub rest_time: i64,
}

impl FromStr for Reindeer {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rest: Vec<&str> = s.split(" can fly ").collect();
        let name = rest[0];
        rest = rest[1].split(" km/s for ").collect();
        let speed = rest[0].parse().unwrap();
        rest = rest[1].split(" seconds, but then must rest for ").collect();
        let stamina = rest[0].parse().unwrap();
        rest = rest[1].split(" seconds.").collect();
        let rest_time = rest[0].parse().unwrap();

        Ok(Reindeer {
            name: name.to_string(),
            speed,
            stamina,
            rest_time,
        })
    }
}

fn get_dist(r: &Reindeer, time: i64) -> i64 {
    let mut fly = r.stamina;
    let mut rest = 0;
    let mut dist = 0;

    for _ in 0..time+1 {
        if fly > 0 {
            dist += r.speed;
            fly -= 1;

            if fly == 0 {
                rest = r.rest_time;
            }
        } else if rest > 0 {
            rest -= 1;
            if rest == 0 {
                fly = r.stamina;
            }
        }
    }

    dist
}

pub fn a() -> i64 {
    let f = BufReader::new(File::open("input/2015/day14.txt").unwrap());
    let mut max = 0;
    for line in f.lines() {
        let line = line.unwrap();
        let r: Reindeer = line.parse().unwrap();
        let dist = get_dist(&r, 2503);
        if dist > max {
            max = dist;
        }
    }

    max
}

#[derive(Debug)]
struct Status {
    reindeer: Reindeer,
    position: i64,
    fly: i64,
    rest: i64,
    points: i64,
}

pub fn b() -> i64 {
    let f = BufReader::new(File::open("input/2015/day14.txt").unwrap());
    let mut s = Vec::new();
    for line in f.lines() {
        let line = line.unwrap();
        let r: Reindeer = line.parse().unwrap();
        let sta = r.stamina;
        s.push(Status{ reindeer: r, position: 0, fly: sta, rest: 0, points: 0 });
    }

    for _ in 0..2503 {
        for s in s.iter_mut() {
            if s.fly > 0 {
                s.position += s.reindeer.speed;
                s.fly -= 1;
                if s.fly == 0 {
                    s.rest = s.reindeer.rest_time;
                }
            } else {
                s.rest -= 1;
                if s.rest == 0 {
                    s.fly = s.reindeer.stamina;
                }
            }
        }

        let max = s.iter().map(|s| s.position).max().unwrap();
        println!("max = {}", max);

        for s in s.iter_mut() {
            if s.position == max {
                s.points += 1;
            }
        }
    }

    s.iter().max_by_key(|&s| s.points).unwrap().points
}