use std::{io::{BufReader, BufRead}, fs::File};

pub fn a() -> i64 {
    let mut ingredients = Vec::new();
    let f = BufReader::new(File::open("input/2015/day15.txt").unwrap());
    for line in f.lines() {
        let line = line.unwrap();

        let mut v: Vec<&str> = line.split(": ").collect();
        let name = v[0];
        v = v[1].split(", ").collect();
        let capacity = v[0][9..].parse::<i64>().unwrap();
        let durability = v[1][11..].parse::<i64>().unwrap();
        let flavor = v[2][7..].parse::<i64>().unwrap();
        let texture = v[3][8..].parse::<i64>().unwrap();
        let calories = v[4][9..].parse::<i64>().unwrap();

        ingredients.push((
            String::from(name),
            capacity,
            durability,
            flavor,
            texture,
            calories
        ));
    }

    find_max_score(&mut Vec::new(), &ingredients, 0)
}

fn score(vars: &[i64], ingredients: &[(String, i64, i64, i64, i64, i64)]) -> i64 {
    let mut capacity = 0;
    let mut durability = 0;
    let mut flavor = 0;
    let mut texture = 0;

    for i in 0..vars.len() {
        capacity += vars[i] * ingredients[i].1;
        durability += vars[i] * ingredients[i].2;
        flavor += vars[i] * ingredients[i].3;
        texture += vars[i] * ingredients[i].4;
    }

    capacity = std::cmp::max(capacity, 0);
    durability = std::cmp::max(durability, 0);
    flavor = std::cmp::max(flavor, 0);
    texture = std::cmp::max(texture, 0);

    capacity * durability * flavor * texture
}

fn count_calories(vars: &mut Vec<i64>, ingredients: &[(String, i64, i64, i64, i64, i64)]) -> i64 {
    let mut sum = 0;
    for i in 0..vars.len() {
        sum += vars[i] * ingredients[i].5;
    }
    sum
}

fn find_max_score(vars: &mut Vec<i64>, ingredients: &[(String, i64, i64, i64, i64, i64)], max_calories: i64) -> i64 {
    let sum: i64 = vars.iter().sum();
    let mut max = 0;

    if max_calories > 0 && count_calories(vars, ingredients) > max_calories {
        return 0;
    }
    if vars.len() == ingredients.len() - 1 {
        vars.push(100 - sum);
        if max_calories > 0 && count_calories(vars, ingredients) > max_calories {
            vars.pop();
            return 0;
        }
        let score = score(vars, ingredients);
        vars.pop();
        return score;
    }

    for i in 0..101 {
        if sum + i > 100 {
            break;
        }

        vars.push(i);
        let score = find_max_score(vars, ingredients, max_calories);
        if score > max {
            max = score;
        }
        vars.pop();
    }

    max
}

pub fn b() -> i64 {
    let mut ingredients = Vec::new();
    let f = BufReader::new(File::open("input/2015/day15.txt").unwrap());
    for line in f.lines() {
        let line = line.unwrap();

        let mut v: Vec<&str> = line.split(": ").collect();
        let name = v[0];
        v = v[1].split(", ").collect();
        let capacity = v[0][9..].parse::<i64>().unwrap();
        let durability = v[1][11..].parse::<i64>().unwrap();
        let flavor = v[2][7..].parse::<i64>().unwrap();
        let texture = v[3][8..].parse::<i64>().unwrap();
        let calories = v[4][9..].parse::<i64>().unwrap();

        ingredients.push((
            String::from(name),
            capacity,
            durability,
            flavor,
            texture,
            calories
        ));
    }

    find_max_score(&mut Vec::new(), &ingredients, 500)
}