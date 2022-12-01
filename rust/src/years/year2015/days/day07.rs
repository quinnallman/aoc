use std::{io::{BufReader, BufRead}, fs::File, collections::HashMap};

fn eval(expr: String, wires: &mut HashMap<String, String>) -> u16 {
    if expr.contains(" AND ") {
        let index = expr.find(" AND ").unwrap();
        let ls = eval(String::from(&expr[..index]), wires);
        let rs = eval(String::from(&expr[index+5..]), wires);
        ls & rs
    } else if expr.contains(" OR ") {
        let index = expr.find(" OR ").unwrap();
        let ls = eval(String::from(&expr[..index]), wires);
        let rs = eval(String::from(&expr[index+4..]), wires);
        ls | rs
    } else if expr.contains(" LSHIFT ") {
        let index = expr.find(" LSHIFT ").unwrap();
        let ls = eval(String::from(&expr[..index]), wires);
        let rs = eval(String::from(&expr[index+8..]), wires);
        ls << rs
    } else if expr.contains(" RSHIFT ") {
        let index = expr.find(" RSHIFT ").unwrap();
        let ls = eval(String::from(&expr[..index]), wires);
        let rs = eval(String::from(&expr[index+8..]), wires);
        ls >> rs
    } else if let Some(expr) = expr.strip_prefix("NOT ") {
        !eval(String::from(expr), wires)
    } else if expr.chars().all(|x| x.is_ascii_lowercase()) {
        let x = eval(String::from(wires.get(&expr).unwrap()), wires);
        let saved = wires.entry(expr).or_default();
        *saved = format!("{}", x);
        x
    } else {
        expr.parse::<u16>().unwrap()
    }
}

pub fn a() -> i64 {
    let f = BufReader::new(File::open("input/2015/day07.txt").unwrap());
    let mut wires = HashMap::<String, String>::new();
    for line in f.lines().map(|x| x.unwrap()) {
        let mut line: Vec<&str> = line.split(" -> ").collect();
        let rs = String::from(line.pop().unwrap());
        let ls = String::from(line.pop().unwrap());
        wires.insert(rs, ls);
    }

    let a_str = wires.get("a").unwrap();
    let ret = eval(String::from(a_str), &mut wires);
    ret as i64
}

pub fn b() -> i64 {
    let f = BufReader::new(File::open("input/2015/day07.txt").unwrap());
    let mut wires = HashMap::<String, String>::new();
    for line in f.lines().map(|x| x.unwrap()) {
        let mut line: Vec<&str> = line.split(" -> ").collect();
        let rs = String::from(line.pop().unwrap());
        let ls = String::from(line.pop().unwrap());
        wires.insert(rs, ls);
    }

    let x = wires.entry(String::from("b")).or_default();
    *x = String::from("16076");

    let a_str = wires.get("a").unwrap();
    let ret = eval(String::from(a_str), &mut wires);
    ret as i64
}