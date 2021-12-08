use std::{fs::File, io::BufReader, io::BufRead, collections::HashMap};
use itertools::Itertools;

pub fn run() -> (i64, i64) {
    (a(), b())
}

fn a() -> i64 {
    let f = BufReader::new(File::open("input/2021/day08.txt").unwrap());
    let mut count = 0;

    for line in f.lines() {
        let line = line.unwrap();
        let mut tokens: Vec<&str> = line.split(" | ").collect();
        let output = tokens.pop().unwrap();
        let tokens: Vec<&str> = output.split_ascii_whitespace().collect();
        for token in tokens {
            if token.len() == 2 || token.len() == 3 || token.len() == 4 || token.len() == 7 {
                count += 1;
            }
        }
    }

    count
}

fn decode(number: &str) -> u8 {
    match number {
        "abcefg" => 0,
        "cf" => 1,
        "acdeg" => 2,
        "acdfg" => 3,
        "bcdf" => 4,
        "abdfg" => 5,
        "abdefg" => 6,
        "acf" => 7,
        "abcdefg" => 8,
        "abcdfg" => 9,
        _ => 99,
    }
}

fn proper(word: &str, map: &HashMap<char, char>) -> String {
    let x = word
        .chars()
        .map(|c| map.get(&c).unwrap())
        .sorted()
        .collect::<String>();
    x
}

fn b() -> i64 {
    let f = BufReader::new(File::open("input/2021/day08.txt").unwrap());

    let mut count = 0;
    for line in f.lines() {
        let line = line.unwrap();
        let mut tokens: Vec<&str> = line.split(" | ").collect();
        let output: Vec<&str> = tokens.pop().unwrap().split_ascii_whitespace().collect();
        let input: Vec<&str> = tokens.pop().unwrap().split_ascii_whitespace().collect();
        let one = *input
            .iter()
            .find(|&&x| x.len() == 2)
            .unwrap();
        let four = *input
            .iter()
            .find(|&&x| x.len() == 4)
            .unwrap();
        let fives: Vec<&str> = input
            .iter()
            .filter(|&&x| x.len() == 5)
            .map(|&x| x)
            .collect();
        let seven = *input
            .iter()
            .find(|&&x| x.len() == 3)
            .unwrap();
        let eight = *input
            .iter()
            .find(|&&x| x.len() == 7)
            .unwrap();

        let chars = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];
        let a = seven
            .chars()
            .filter(|x| !one.contains(*x))
            .next()
            .unwrap();
        let d = four
            .chars()
            .filter(|&c| fives.iter().all(|&f| f.contains(c)))
            .next()
            .unwrap();
        let g = *chars
            .iter()
            .filter(|&&c| fives.iter().all(|&f| f.contains(c)) && c != a && c != d)
            .next()
            .unwrap();
        let b = *chars
            .iter()
            .filter(|&&c| four.contains(c) && !one.contains(c) && c != d)
            .next()
            .unwrap();
        let five = *fives
            .iter()
            .find(|&&x| x.contains(b))
            .unwrap();
        let f = five
            .chars()
            .filter(|&c| c != a && c != b && c != d && c != g)
            .next()
            .unwrap();
        let c = one
            .chars()
            .filter(|&c| c != f)
            .next()
            .unwrap();
        let e = eight
            .chars()
            .filter(|&ch| ch != a && ch != b && ch != c && ch != d && ch != f && ch != g)
            .next()
            .unwrap();
        
        let mut map: HashMap<char, char> = HashMap::new();
        map.insert(a, 'a');
        map.insert(b, 'b');
        map.insert(c, 'c');
        map.insert(d, 'd');
        map.insert(e, 'e');
        map.insert(f, 'f');
        map.insert(g, 'g');

        let x = output
            .iter()
            .map(|&o| {
                let x = &proper(o, &map)[..];
                return decode(x) as u32;
            })
            .reduce(|a, b| a*10+b)
            .unwrap();
        count += x;
    }

    count as i64
}
