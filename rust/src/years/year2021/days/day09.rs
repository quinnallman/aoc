use std::{fs::File, io::BufReader, io::BufRead};

use itertools::Itertools;

fn get_low_points(map: &[[i64; 100]; 100]) -> Vec<(usize, usize)> {
    const W: usize = 100;
    const H: usize = 100;
    let mut ret = Vec::new();

    // check each corner
    if map[0][0] < map[0][1] && map[0][0] < map[1][0] {
        ret.push((0, 0));
    }
    if map[0][W-1] < map[0][W-2] && map[0][W-1] < map[1][W-1] {
        ret.push((0, W-1));
    }
    if map[H-1][0] < map[H-2][0] && map[H-1][0] < map[H-1][1] {
        ret.push((H-1, 0));
    }
    if map[H-1][W-1] < map[H-1][W-2] && map[H-1][W-1] < map[H-2][W-1] {
        ret.push((H-1, W-1));
    }

    // check the first row
    for j in 1..W-2 {
        if map[0][j] < map[0][j-1] && map[0][j] < map[0][j+1] && map[0][j] < map[1][j] {
            ret.push((0, j));
        }
    }

    // check the last row
    for j in 1..W-2 {
        if map[H-1][j] < map[H-1][j-1] && map[H-1][j] < map[H-1][j+1] && map[H-1][j] < map[H-2][j] {
            ret.push((H-1, j));
        }
    }

    // check the first column
    for i in 1..H-2 {
        if map[i][0] < map[i-1][0] && map[i][0] < map[i+1][0] && map[i][0] < map[i][1] {
            ret.push((i, 0));
        }
    }

    // check the last column
    for i in 1..H-2 {
        if map[i][W-1] < map[i-1][W-1] && map[i][W-1] < map[i+1][W-1] && map[i][W-1] < map[i][W-2] {
            ret.push((i, W-1));
        }
    }

    for i in 1..H-1 {
        for j in 1..W-1 {
            if map[i][j] < map[i-1][j] && map[i][j] < map[i][j-1] && map[i][j] < map[i][j+1] && map[i][j] < map[i+1][j] {
                ret.push((i, j));
            }
        }
    }

    ret
}

pub fn a() -> i64 {
    let f = BufReader::new(File::open("input/2021/day09.txt").unwrap());

    const W: usize = 100;
    const H: usize = 100;
    let mut heightmap: [[i64; W]; H] = [[0; W]; H];

    for (i, line) in f.lines().enumerate() {
        let mut j = 0;
        line
            .unwrap()
            .chars()
            .for_each(|x| {
                let height = x as i64 - '0' as i64;
                heightmap[i][j] = height;
                j += 1;
            });
    }

    let mut sum = 0;

    let low_points = get_low_points(&heightmap);
    for low in low_points {
        sum += 1 + heightmap[low.0][low.1];
    }

    sum
}

pub fn b() -> i64 {
    let f = BufReader::new(File::open("input/2021/day09.txt").unwrap());

    const W: usize = 100;
    const H: usize = 100;
    let mut heightmap: [[i64; W]; H] = [[0; W]; H];

    for (i, line) in f.lines().enumerate() {
        let mut j = 0;
        line
            .unwrap()
            .chars()
            .for_each(|x| {
                let height = x as i64 - '0' as i64;
                heightmap[i][j] = height;
                j += 1;
            });
    }

    let low_points = get_low_points(&heightmap);
    let mut basins = Vec::new();

    for low in low_points {
        let mut visited = Vec::new();
        let mut q = vec![low];

        while !q.is_empty() {
            let p = q.pop().unwrap();
            if heightmap[p.0][p.1] == 9 || visited.contains(&p) {
                continue;
            }

            visited.push(p);

            // check left
            if p.1 > 0 && heightmap[p.0][p.1-1] >= heightmap[p.0][p.1] {
                q.push((p.0, p.1-1));
            }
            // check right
            if p.1 < W-1 && heightmap[p.0][p.1+1] >= heightmap[p.0][p.1] {
                q.push((p.0, p.1+1));
            }
            // check above
            if p.0 > 0 && heightmap[p.0-1][p.1] >= heightmap[p.0][p.1] {
                q.push((p.0-1, p.1));
            }
            // check below
            if p.0 < H-1 && heightmap[p.0+1][p.1] >= heightmap[p.0][p.1] {
                q.push((p.0+1, p.1));
            }
        }

        basins.push(visited.len());
    }

    let mut prod = 1;
    let mut basins: Vec<&usize> = basins.iter().sorted().collect();
    for _ in 0..3 {
        prod *= *basins.pop().unwrap();
    }

    prod as i64
}
