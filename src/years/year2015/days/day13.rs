use std::{io::{BufReader, BufRead}, fs::File, collections::HashMap};

#[derive(Debug)]
struct HappinessChange {
    name1: String,
    name2: String,
    amount: i64,
}

fn parse_line(line: &mut str) -> HappinessChange {
    let mut tokens = line.split_ascii_whitespace();

    let name1 = String::from(tokens.next().unwrap());
    tokens.next();

    let gain_or_lose = tokens.next().unwrap();
    let mut amount = tokens.next().unwrap().parse::<i64>().unwrap();
    if gain_or_lose == "lose" {
        amount *= -1;
    }

    tokens.next();
    tokens.next();
    tokens.next();
    tokens.next();
    tokens.next();
    tokens.next();

    let name = tokens.next().unwrap();
    let name2 = name.chars().take(name.len() - 1).collect::<String>();

    HappinessChange {
        name1,
        name2,
        amount,
    }
}

fn get_index(name: &str) -> usize {
    match name {
        "Alice" => 0,
        "Bob" => 1,
        "Carol" => 2,
        "David" => 3,
        "Eric" => 4,
        "Frank" => 5,
        "George" => 6,
        "Mallory" => 7,
        _ => 0,
    }
}

fn left(i: usize) -> usize {
    if i == 0 {
        3
    } else {
        i-1
    }
}

fn max_happiness(table: &mut Vec<usize>, seated: &mut Vec<bool>, h: &[Vec<i64>]) -> i64 {
    if table.len() == h[0].len() {
        let mut sum = 0;
        for (i, p) in table.iter().enumerate() {
            let left = match i { 0 => table[table.len() - 1], _ => table[i-1] };
            if i == table.len() - 1 {
                sum += h[*p][left] + h[*p][table[0]];
            } else {
                sum += h[*p][left] + h[*p][table[i+1]];
            }
        }

        sum
    } else {
        let mut v = Vec::new();
        let len = h[0].len();

        for i in 0..len {
            if seated[i] {
                continue;
            }

            seated[i] = true;
            table.push(i);
            let x = max_happiness(table, seated, h);
            v.push(x);
            table.pop();
            seated[i] = false;
        }

        *v.iter().max().unwrap()
    }
}

fn record_happiness(h: &mut Vec<Vec<i64>>, diners: &HashMap<String, usize>, hc: &HappinessChange) {
    let row = *diners.get(&hc.name1).unwrap();
    let col = *diners.get(&hc.name2).unwrap();
    h[row][col] = hc.amount;
}

pub fn a() -> i64 {
    let f = BufReader::new(File::open("input/2015/day13.txt").unwrap());
    let mut map: HashMap<String, usize> = HashMap::new();
    let mut num_diners = 0;
    let mut hc: Vec<HappinessChange> = Vec::new();

    for line in f.lines() {
        let mut line = line.unwrap();
        let entry = parse_line(&mut line[..]);

        if !map.contains_key(&entry.name1) {
            map.insert(entry.name1.clone(), num_diners);
            num_diners += 1;
        }
        if !map.contains_key(&entry.name2) {
            map.insert(entry.name2.clone(), num_diners);
            num_diners += 1;
        }

        hc.push(entry);
    }

    let mut h: Vec<Vec<i64>> = Vec::with_capacity(num_diners);
    for _ in 0..num_diners {
        h.push(vec![0; num_diners]);
    }

    for hc in hc.iter() {
        record_happiness(&mut h, &map, hc);
    }

    let mut seated = vec![false; num_diners];
    max_happiness(&mut Vec::new(), &mut seated, &h)
}

pub fn b() -> i64 {
    let f = BufReader::new(File::open("input/2015/day13.txt").unwrap());
    let mut map: HashMap<String, usize> = HashMap::new();
    let mut num_diners = 0;
    let mut hc: Vec<HappinessChange> = Vec::new();

    for line in f.lines() {
        let mut line = line.unwrap();
        let entry = parse_line(&mut line[..]);

        if !map.contains_key(&entry.name1) {
            map.insert(entry.name1.clone(), num_diners);
            num_diners += 1;
        }
        if !map.contains_key(&entry.name2) {
            map.insert(entry.name2.clone(), num_diners);
            num_diners += 1;
        }

        hc.push(entry);
    }

    map.insert(String::from("Me"), num_diners);
    num_diners += 1;

    let mut h: Vec<Vec<i64>> = Vec::with_capacity(num_diners);
    for _ in 0..num_diners {
        h.push(vec![0; num_diners]);
    }

    for hc in hc.iter() {
        record_happiness(&mut h, &map, hc);
    }

    let mut seated = vec![false; num_diners];
    max_happiness(&mut Vec::new(), &mut seated, &h)
}