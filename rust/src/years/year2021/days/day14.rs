use std::{io::{BufReader, BufRead}, fs::File, collections::HashMap};

use itertools::Itertools;

pub fn a() -> i64 {
    let f = BufReader::new(File::open("input/2021/day14.txt").unwrap());
    let mut iter = f.lines();
    let template = iter.next().unwrap().unwrap();
    iter.next();

    let mut rules: HashMap<String, String> = HashMap::new();
    for line in iter {
        let line = line.unwrap();
        let rule: Vec<&str> = line.split(" -> ").collect();

        let mut result = String::from(rule[0]);
        result.insert(1, rule[1].chars().next().unwrap());

        rules.insert(String::from(rule[0]), result);
    }

    let mut counts = count_tuples(&template[..]);

    for _ in 0..10 {
        counts = translate(&counts, &rules);
    }

    let mut char_counts: HashMap<char, i64> = HashMap::new();
    for c in counts {
        let v: Vec<char> = c.0.chars().collect();
        let mut cc = char_counts.entry(v[0]).or_insert(0);
        *cc += c.1;
        cc = char_counts.entry(v[1]).or_insert(0);
        *cc += c.1;
    }

    // every character in the string (except the first and last)
    // is part of 2 pairs, so let's divide each one, accounting
    // for the missing first and last characters (n.b. the first
    // and last characters don't ever change so we can reference
    // the original template for those
    for (c, count) in char_counts.iter_mut() {
        if template.starts_with(*c) {
            *count += 1;
        }
        if template.ends_with(*c) {
            *count += 1;
        }

        *count /= 2;
    }

    let (_, max_i) = char_counts.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap();
    let (_, min_i) = char_counts.iter().min_by(|a, b| a.1.cmp(b.1)).unwrap();
    max_i - min_i
}

fn count_tuples(template: &str) -> HashMap<String, i64> {
    let mut counts: HashMap<String, i64> = HashMap::new();
    for (c1, c2) in template.chars().tuple_windows() {
        let mut s = String::from(c1);
        s.push(c2);
        let entry = counts.entry(s).or_insert(0);
        *entry += 1;
    }
    counts
}

fn translate(counts: &HashMap<String, i64>, rules: &HashMap<String, String>) -> HashMap<String, i64> {
    let mut new_counts: HashMap<String, i64> = HashMap::new();
    for (k, v) in counts {
        if rules.contains_key(k) {
            let result = rules.get(k).unwrap();
            let e1 = String::from(&result[..2]);
            let e2 = String::from(&result[1..]);
            let mut entry = new_counts.entry(e1).or_insert(0);
            *entry += v;
            entry = new_counts.entry(e2).or_insert(0);
            *entry += v;
        } else {
            let entry = new_counts.entry(k.clone()).or_insert(0);
            *entry += v;
        }
    }

    new_counts
}

pub fn b() -> i64 {
    let f = BufReader::new(File::open("input/2021/day14.txt").unwrap());
    let mut iter = f.lines();
    let template = iter.next().unwrap().unwrap();
    iter.next();

    let mut rules: HashMap<String, String> = HashMap::new();
    for line in iter {
        let line = line.unwrap();
        let rule: Vec<&str> = line.split(" -> ").collect();

        let mut result = String::from(rule[0]);
        result.insert(1, rule[1].chars().next().unwrap());

        rules.insert(String::from(rule[0]), result);
    }

    let mut counts = count_tuples(&template[..]);

    for _ in 0..40 {
        counts = translate(&counts, &rules);
    }

    let mut char_counts: HashMap<char, i64> = HashMap::new();
    for c in counts {
        let v: Vec<char> = c.0.chars().collect();
        let mut cc = char_counts.entry(v[0]).or_insert(0);
        *cc += c.1;
        cc = char_counts.entry(v[1]).or_insert(0);
        *cc += c.1;
    }

    // every character in the string (except the first and last)
    // is part of 2 pairs, so let's divide each one, accounting
    // for the missing first and last characters (n.b. the first
    // and last characters don't ever change so we can reference
    // the original template for those
    for (c, count) in char_counts.iter_mut() {
        if template.starts_with(*c) {
            *count += 1;
        }
        if template.ends_with(*c) {
            *count += 1;
        }

        *count /= 2;
    }

    let (_, max_i) = char_counts.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap();
    let (_, min_i) = char_counts.iter().min_by(|a, b| a.1.cmp(b.1)).unwrap();
    max_i - min_i
}