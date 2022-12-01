use std::{io::{BufRead, BufReader}, fs::File, collections::HashMap};

pub fn a() -> i64 {
    let f = BufReader::new(File::open("input/2021/day12.txt").unwrap());
    let mut map = HashMap::new();
    let mut connections = Vec::new();
    let mut num_nodes = 0;
    let mut big_caves = Vec::new();
    for line in f.lines() {
        let line = line.unwrap();
        let line: Vec<&str> = line.split('-').collect();
        let p1 = String::from(line[0]);
        let p2 = String::from(line[1]);

        if !map.contains_key(&p1) {
            map.insert(p1.clone(), num_nodes);
            if p1.to_uppercase() == p1 {
                big_caves.push(num_nodes);
            }
            num_nodes += 1;
        }
        if !map.contains_key(&p2) {
            map.insert(p2.clone(), num_nodes);
            if p2.to_uppercase() == p2 {
                big_caves.push(num_nodes);
            }
            num_nodes += 1;
        }

        connections.push((p1, p2));
    }

    //println!("{:?}", map);
    //println!("{:?}", big_caves);

    let mut adj: Vec<Vec<i64>> = Vec::with_capacity(num_nodes);
    for _ in 0..num_nodes {
        adj.push(vec![0; num_nodes]);
    }

    for c in connections {
        let i = map.get(&c.0).unwrap();
        let j = map.get(&c.1).unwrap();

        adj[*i][*j] = 1;
        adj[*j][*i] = 1;
    }

    let mut path = Vec::new();
    let start = *map.get("start").unwrap();
    path.push(start);
    count_paths(0, &mut path, &adj, &big_caves, &map)
}

fn num_revisited(path: &[usize], map: &HashMap<String, usize>) -> i64 {
    let mut h: HashMap<usize, i64> = HashMap::new();
    for p in path {
        for (m, i) in map {
            if *i == *p {
                if m.to_lowercase() == *m {
                    let e = h.entry(*p).or_insert(0);
                    *e += 1;
                } else {
                    break;
                }
            }
        }
    }

    h.iter().map(|(_, v)| *v).filter(|&v| v > 1).sum()
}

fn count_paths(k: i64, path: &mut Vec<usize>, adj: &[Vec<i64>], big_caves: &[usize], map: &HashMap<String, usize>) -> i64 {
    let end = path[path.len() - 1];
    if end == *map.get("end").unwrap() {
        //println!("{:?}", path);
        return 1;
    }

    let mut sum = 0;
    for (i, v) in adj[end].iter().enumerate() {
        if *v == 1 && i != *map.get("start").unwrap() && (!path.contains(&i) || big_caves.contains(&i) || num_revisited(path, map) < k) {
            path.push(i);
            sum += count_paths(k, path, adj, big_caves, map);
            path.pop();
        }
    }

    sum
}

pub fn b() -> i64 {
    let f = BufReader::new(File::open("input/2021/day12.txt").unwrap());
    let mut map = HashMap::new();
    let mut connections = Vec::new();
    let mut num_nodes = 0;
    let mut big_caves = Vec::new();
    for line in f.lines() {
        let line = line.unwrap();
        let line: Vec<&str> = line.split('-').collect();
        let p1 = String::from(line[0]);
        let p2 = String::from(line[1]);

        if !map.contains_key(&p1) {
            map.insert(p1.clone(), num_nodes);
            if p1.to_uppercase() == p1 {
                big_caves.push(num_nodes);
            }
            num_nodes += 1;
        }
        if !map.contains_key(&p2) {
            map.insert(p2.clone(), num_nodes);
            if p2.to_uppercase() == p2 {
                big_caves.push(num_nodes);
            }
            num_nodes += 1;
        }

        connections.push((p1, p2));
    }

    //println!("{:?}", map);
    //println!("{:?}", big_caves);

    let mut adj: Vec<Vec<i64>> = Vec::with_capacity(num_nodes);
    for _ in 0..num_nodes {
        adj.push(vec![0; num_nodes]);
    }

    for c in connections {
        let i = map.get(&c.0).unwrap();
        let j = map.get(&c.1).unwrap();

        adj[*i][*j] = 1;
        adj[*j][*i] = 1;
    }

    let mut path = Vec::new();
    let start = *map.get("start").unwrap();
    path.push(start);
    count_paths(1, &mut path, &adj, &big_caves, &map)
}