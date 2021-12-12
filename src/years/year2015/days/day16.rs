use std::{io::{BufReader, BufRead}, fs::File};

#[derive(Debug, Default)]
struct Sue {
    number: i64,
    children: i64,
    cats: i64,
    samoyeds: i64,
    pomeranians: i64,
    akitas: i64,
    vizslas: i64,
    goldfish: i64,
    trees: i64,
    cars: i64,
    perfumes: i64,
}

pub fn a() -> i64 {
    let f = BufReader::new(File::open("input/2015/day16.txt").unwrap());
    let mut sues: Vec<Sue> = Vec::new();

    for line in f.lines() {
        let mut sue: Sue = Sue{number: 0, children: -1, cats: -1, samoyeds: -1, pomeranians: -1, akitas: -1, vizslas: -1, goldfish: -1, trees: -1, cars: -1, perfumes: -1};
        let line = &line.unwrap()[4..];
        let v = line.split_once(": ").unwrap();
        let number = v.0.parse::<i64>().unwrap();
        sue.number = number;
        let v: Vec<&str> = v.1.split(", ").collect();
        //let mut things: HashMap<String, i64> = HashMap::new();
        for v in v {
            let v: Vec<&str> = v.split(": ").collect();
            let num = v[1].parse::<i64>().unwrap();
            match v[0] {
                "children" => { sue.children = num },
                "cats" => { sue.cats = num },
                "samoyeds" => { sue.samoyeds = num },
                "pomeranians" => { sue.pomeranians = num },
                "akitas" => { sue.akitas = num },
                "vizslas" => { sue.vizslas = num },
                "goldfish" => { sue.goldfish = num },
                "trees" => { sue.trees = num },
                "cars" => { sue.cars = num },
                "perfumes" => { sue.perfumes = num },
                _ => {},
            }
        }

        sues.push(sue);
    }

    sues = sues
        .into_iter()
        .filter(|x| x.children == -1 || x.children == 3)
        .filter(|x| x.cats == -1 || x.cats == 7)
        .filter(|x| x.samoyeds == -1 || x.samoyeds == 2)
        .filter(|x| x.pomeranians == -1 || x.pomeranians == 3)
        .filter(|x| x.akitas == -1 || x.akitas == 0)
        .filter(|x| x.vizslas == -1 || x.vizslas == 0)
        .filter(|x| x.goldfish == -1 || x.goldfish == 5)
        .filter(|x| x.trees == -1 || x.trees == 3)
        .filter(|x| x.cars == -1 || x.cars == 2)
        .filter(|x| x.perfumes == -1 || x.perfumes == 1)
        .collect();

    sues[0].number
}

pub fn b() -> i64 {
    let f = BufReader::new(File::open("input/2015/day16.txt").unwrap());
    let mut sues: Vec<Sue> = Vec::new();

    for line in f.lines() {
        let mut sue: Sue = Sue{number: 0, children: -1, cats: -1, samoyeds: -1, pomeranians: -1, akitas: -1, vizslas: -1, goldfish: -1, trees: -1, cars: -1, perfumes: -1};
        let line = &line.unwrap()[4..];
        let v = line.split_once(": ").unwrap();
        let number = v.0.parse::<i64>().unwrap();
        sue.number = number;
        let v: Vec<&str> = v.1.split(", ").collect();
        //let mut things: HashMap<String, i64> = HashMap::new();
        for v in v {
            let v: Vec<&str> = v.split(": ").collect();
            let num = v[1].parse::<i64>().unwrap();
            match v[0] {
                "children" => { sue.children = num },
                "cats" => { sue.cats = num },
                "samoyeds" => { sue.samoyeds = num },
                "pomeranians" => { sue.pomeranians = num },
                "akitas" => { sue.akitas = num },
                "vizslas" => { sue.vizslas = num },
                "goldfish" => { sue.goldfish = num },
                "trees" => { sue.trees = num },
                "cars" => { sue.cars = num },
                "perfumes" => { sue.perfumes = num },
                _ => {},
            }
        }

        sues.push(sue);
    }

    sues = sues
        .into_iter()
        .filter(|x| x.children == -1 || x.children == 3)
        .filter(|x| x.cats == -1 || x.cats > 7)
        .filter(|x| x.samoyeds == -1 || x.samoyeds == 2)
        .filter(|x| x.pomeranians == -1 || x.pomeranians < 3)
        .filter(|x| x.akitas == -1 || x.akitas == 0)
        .filter(|x| x.vizslas == -1 || x.vizslas == 0)
        .filter(|x| x.goldfish == -1 || x.goldfish < 5)
        .filter(|x| x.trees == -1 || x.trees > 3)
        .filter(|x| x.cars == -1 || x.cars == 2)
        .filter(|x| x.perfumes == -1 || x.perfumes == 1)
        .collect();

    sues[0].number
}