use std::{io::{BufReader, BufRead}, fs::File};

pub fn a() -> i64 {
    let f = BufReader::new(File::open("input/2021/day13.txt").unwrap());
    let mut coords = Vec::new();
    let mut folds = Vec::new();
    for line in f.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            continue;
        } else if let Some(line) = line.strip_prefix("fold along ") {
            let fold: Vec<&str> = line.split('=').collect();
            folds.push((String::from(fold[0]), fold[1].parse::<i64>().unwrap()));
            continue;
        } else {
            let coord: Vec<i64> = line.split(',').map(|x| x.parse().unwrap()).collect();
            coords.push((coord[0], coord[1]));
        }
    }

    let mut result;
    result = fold(&coords, (folds[0].0.clone(), folds[0].1));
    let min_x = result.iter().map(|(x, _)| *x).min().unwrap();
    let min_y = result.iter().map(|(_, y)| *y).min().unwrap();
    for (i, j) in result.iter_mut() {
        if folds[0].0 == "x" && min_x < 0 {
            *i += min_x;
        } else if folds[0].0 == "y" && min_y < 0 {
            *j += min_y
        }
    }

    //print_grid(&result);

    result.len() as i64
}

fn print_grid(coords: &[(i64, i64)]) {
    let max_y = coords.iter().map(|(_, y)| *y).max().unwrap();
    let max_x = coords.iter().map(|(x, _)| *x).max().unwrap();
    for i in 0..max_y+1 {
        for j in 0..max_x+1 {
            if coords.contains(&(j, i)) {
                print!("█");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn fold(coords: &[(i64, i64)], fold: (String, i64)) -> Vec<(i64, i64)> {
    let mut result: Vec<(i64, i64)> = Vec::new();
    for coord in coords {
        if fold.0 == "x" {
            if coord.0 < fold.1 {
                if !result.contains(coord) {
                    result.push((coord.0, coord.1));
                }
            } else {
                let mirror = (coord.0 - 2 * (coord.0 - fold.1), coord.1);
                if !result.contains(&mirror) {
                    result.push(mirror);
                }
            }
        } else if coord.1 < fold.1 {
            if !result.contains(coord) {
                result.push((coord.0, coord.1));
            }
        } else {
            let mirror = (coord.0, coord.1 - 2 * (coord.1 - fold.1));
            if !result.contains(&mirror) {
                result.push(mirror);
            }
        }
    }

    result
}

pub fn b() -> i64 {
    let f = BufReader::new(File::open("input/2021/day13.txt").unwrap());
    let mut coords = Vec::new();
    let mut folds = Vec::new();
    for line in f.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            continue;
        } else if let Some(line) = line.strip_prefix("fold along ") {
            let fold: Vec<&str> = line.split('=').collect();
            folds.push((String::from(fold[0]), fold[1].parse::<i64>().unwrap()));
            continue;
        } else {
            let coord: Vec<i64> = line.split(',').map(|x| x.parse().unwrap()).collect();
            coords.push((coord[0], coord[1]));
        }
    }

    let mut result = coords.clone();

    for curr in folds {
        result = fold(&result.clone(), (curr.0.clone(), curr.1));
        let min_x = result.iter().map(|(x, _)| *x).min().unwrap();
        let min_y = result.iter().map(|(_, y)| *y).min().unwrap();
        for (i, j) in result.iter_mut() {
            if curr.0 == "x" && min_x < 0 {
                *i += min_x;
            } else if curr.0 == "y" && min_y < 0 {
                *j += min_y
            }
        }
    }

    print_grid(&result);

    result.len() as i64
}