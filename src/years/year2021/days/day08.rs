use std::{fs::File, io::BufReader, io::BufRead, collections::HashMap};
use itertools::Itertools;

pub fn a() -> i64 {
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

pub fn b() -> i64 {
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
            .copied()
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
        let seg_a = seven
            .chars()
            .find(|&x| !one.contains(x))
            .unwrap();
        let seg_d = four
            .chars()
            .find(|&c| fives.iter().all(|&f| f.contains(c)))
            .unwrap();
        let seg_g = *chars
            .iter()
            .find(|&&c| fives.iter().all(|&f| f.contains(c)) && c != seg_a && c != seg_d)
            .unwrap();
        let seg_b = *chars
            .iter()
            .find(|&&c| four.contains(c) && !one.contains(c) && c != seg_d)
            .unwrap();
        let five = *fives
            .iter()
            .find(|&&x| x.contains(seg_b))
            .unwrap();
        let seg_f = five
            .chars()
            .find(|&c| c != seg_a && c != seg_b && c != seg_d && c != seg_g)
            .unwrap();
        let seg_c = one
            .chars()
            .find(|&ch| ch != seg_f)
            .unwrap();
        let seg_e = eight
            .chars()
            .find(|&ch| ch != seg_a && ch != seg_b && ch != seg_c && ch != seg_d && ch != seg_f && ch != seg_g)
            .unwrap();
        
        let mut map: HashMap<char, char> = HashMap::new();
        map.insert(seg_a, 'a');
        map.insert(seg_b, 'b');
        map.insert(seg_c, 'c');
        map.insert(seg_d, 'd');
        map.insert(seg_e, 'e');
        map.insert(seg_f, 'f');
        map.insert(seg_g, 'g');

        let x = output
            .iter()
            .map(|&o| {
                let x = &proper(o, &map)[..];
                decode(x) as u32
            })
            .reduce(|a, b| a*10+b)
            .unwrap();
        count += x;
    }

    count as i64
}
