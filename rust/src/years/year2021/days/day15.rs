use std::{io::{BufReader, BufRead}, fs::File, collections::{HashMap, BinaryHeap}};

fn reconstruct_path(came_from: &HashMap<(i64, i64), (i64, i64)>, current: &(i64, i64)) -> Vec<(i64, i64)> {
    let mut total_path = vec![*current];

    let mut c = *current;
    loop {
        if !came_from.contains_key(&c) {
            break;
        }
        c = *came_from.get(&c).unwrap();
        total_path.insert(0, c);
    }

    total_path
}

fn pathfind(start: &(i64, i64), goal: &(i64, i64), graph: &[Vec<i64>]) -> Vec<(i64, i64)> {
    let mut open_set: BinaryHeap<(i64, i64, i64)> = BinaryHeap::new();
    open_set.push((0, start.0, start.1));
    let mut came_from: HashMap<(i64, i64), (i64, i64)> = HashMap::new();
    let mut g_score: HashMap<(i64, i64), i64> = HashMap::new();
    for i in 0..graph.len() {
        for j in 0..graph[0].len() {
            g_score.insert((i as i64, j as i64), i64::MAX);
        }
    }

    let g = g_score.entry(*start).or_insert(0);
    *g = 0;

    let mut f_score: HashMap<(i64, i64), i64> = HashMap::new();
    for i in 0..graph.len() {
        for j in 0..graph[0].len() {
            f_score.insert((i as i64, j as i64), i64::MAX);
        }
    }
    let f = f_score.entry(*start).or_insert(0);
    *f = (goal.0 - start.0).abs() + (goal.1 - start.1).abs();

    let g = g_score.entry(*start).or_insert(0);
    *g = 0;

    while !open_set.is_empty() {
        let current_p = open_set.pop().unwrap();
        let current = (current_p.1, current_p.2);
        if current == *goal {
            return reconstruct_path(&came_from, &current);
        }

        let mut neighbours = Vec::new();
        if current.0 >= 1 {
            neighbours.push((current.0-1, current.1));
        }
        if current.0 < (graph.len() - 1) as i64 {
            neighbours.push((current.0+1, current.1));
        }
        if current.1 >= 1 {
            neighbours.push((current.0, current.1-1));
        }
        if current.1 < (graph[0].len() - 1) as i64 {
            neighbours.push((current.0, current.1+1));
        }

        for neighbour in neighbours {
            let g = graph[neighbour.0 as usize][neighbour.1 as usize];
            let gscore = *g_score.entry(current).or_default();
            let gscore_neighbour = g_score.entry(neighbour).or_default();
            let fscore_neighbour = f_score.entry(neighbour).or_default();
            let tentative_gscore = gscore + g;
            if tentative_gscore < *gscore_neighbour {
                let camefrom = came_from.entry(neighbour).or_default();
                *camefrom = current;
                *gscore_neighbour = tentative_gscore;
                *fscore_neighbour = tentative_gscore + (goal.0 - neighbour.0).abs() + (goal.1 - neighbour.1).abs();
                let mut found = false;
                for os in open_set.iter() {
                    if os.1 == neighbour.0 && os.2 == neighbour.1 {
                        found = true;
                        break;
                    }
                }
                if !found {
                    open_set.push((-*fscore_neighbour, neighbour.0, neighbour.1));
                }
            }
        }
    }

    vec![*start]
}

pub fn a() -> i64 {
    let f = BufReader::new(File::open("input/2021/day15.txt").unwrap());
    let mut graph: Vec<Vec<i64>> = Vec::new();

    let mut width: i64 = 0;
    let mut height: i64 = 0;

    for line in f.lines() {
        let line = line.unwrap();
        let v: Vec<i64> = line.chars().map(|c| (c as u8 - b'0') as i64).collect();

        if width == 0 {
            width = v.len() as i64;
        }
        height += 1;

        graph.push(v);
    }

    let path = pathfind(&(0, 0), &((height - 1) as i64, (width - 1) as i64), &graph);
    println!("{:?}", path);
    let mut risk = 0;
    for p in path {
        risk += graph[p.0 as usize][p.1 as usize];
    }

    risk - graph[0][0]
}

pub fn b() -> i64 {
    let f = BufReader::new(File::open("input/2021/day15.txt").unwrap());
    let mut graph: Vec<Vec<i64>> = Vec::new();

    let mut width: i64 = 0;
    let mut height: i64 = 0;

    for line in f.lines() {
        let line = line.unwrap();
        let v: Vec<i64> = line.chars().map(|c| (c as u8 - b'0') as i64).collect();

        if width == 0 {
            width = v.len() as i64;
        }
        height += 1;

        graph.push(v);
    }

    let graph = (0..(5*graph.len()))
        .map(|y| (0..(5*graph[0].len()))
            .map(|x| {
                let ny = y / graph.len();
                let nx = x / graph[0].len();
                let risk = graph[y%graph.len()][x%graph[0].len()] + ny as i64 + nx as i64;
                if risk < 10 {
                    risk
                } else {
                    risk - 9
                }
            })
            .collect::<Vec<i64>>()
        ).collect::<Vec<Vec<i64>>>();

    let path = pathfind(&(0, 0), &((height*5 - 1) as i64, (width*5 - 1) as i64), &graph);

    let mut risk = 0;
    for p in path {
        risk += graph[p.0 as usize][p.1 as usize];
    }

    risk - graph[0][0]
}