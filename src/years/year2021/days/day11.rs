use std::{io::{BufReader, BufRead}, fs::File};

fn nice_print(v: &[Vec<i64>]) {
    for row in v {
        for cell in row {
            let disp = match *cell {
                1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 => *cell,
                _ => 0,
            };
            print!("{}", disp);
        }
        println!();
    }
    println!();
}

pub fn a() -> i64 {
    const MAX_STEPS: i64 = 100;

    let f = BufReader::new(File::open("input/2021/day11.txt").unwrap());
    let mut v = Vec::new();
    for line in f.lines() {
        let line = line.unwrap();
        let mut v2 = Vec::new();
        for c in line.chars() {
            v2.push(c as i64 - '0' as i64);
        }
        v.push(v2);
    }

    //println!("Before any steps:");
    //nice_print(&v);

    let mut sum = 0;
    for _ in 0..MAX_STEPS {
        let mut flashed = Vec::new();
        for (i, row) in v.iter_mut().enumerate() {
            for (j, cell) in row.iter_mut().enumerate() {
                *cell += 1;
                if *cell > 9 {
                    flashed.push((i, j));
                }
            }
        }

        let mut handled = Vec::new();
        while !flashed.is_empty() {
            let curr = flashed.pop().unwrap();
            handled.push(curr);
            sum += 1;
            for o_y in -1_i64..2 {
                for o_x in -1_i64..2 {
                    if o_y == 0 && o_x == 0 {
                        continue;
                    }
                    let row = curr.0 as i64 + o_y;
                    let col = curr.1 as i64 + o_x;
                    if row >= 0 && row < v.len() as i64 && col >= 0 && col < v[row as usize].len() as i64 {
                        v[row as usize][col as usize] += 1;
                        if v[row as usize][col as usize] > 9 && !handled.contains(&(row as usize, col as usize)) && !flashed.contains(&(row as usize, col as usize)) {
                            flashed.push((row as usize, col as usize));
                        }
                    }
                }
            }
        }

        for h in handled {
            v[h.0][h.1] = 0;
        }

        //println!("After step {}:", i+1);
        //nice_print(&v);
    }

    sum
}

pub fn b() -> i64 {
    const MAX_STEPS: i64 = 100;

    let f = BufReader::new(File::open("input/2021/day11.txt").unwrap());
    let mut v = Vec::new();
    for line in f.lines() {
        let line = line.unwrap();
        let mut v2 = Vec::new();
        for c in line.chars() {
            v2.push(c as i64 - '0' as i64);
        }
        v.push(v2);
    }

    //println!("Before any steps:");
    //nice_print(&v);

    let mut step = 1;
    loop {
        let mut flashed = Vec::new();
        for (i, row) in v.iter_mut().enumerate() {
            for (j, cell) in row.iter_mut().enumerate() {
                *cell += 1;
                if *cell > 9 {
                    flashed.push((i, j));
                }
            }
        }

        let mut handled = Vec::new();
        while !flashed.is_empty() {
            let curr = flashed.pop().unwrap();
            handled.push(curr);

            for o_y in -1_i64..2 {
                for o_x in -1_i64..2 {
                    if o_y == 0 && o_x == 0 {
                        continue;
                    }
                    let row = curr.0 as i64 + o_y;
                    let col = curr.1 as i64 + o_x;
                    if row >= 0 && row < v.len() as i64 && col >= 0 && col < v[row as usize].len() as i64 {
                        v[row as usize][col as usize] += 1;
                        if v[row as usize][col as usize] > 9 && !handled.contains(&(row as usize, col as usize)) && !flashed.contains(&(row as usize, col as usize)) {
                            flashed.push((row as usize, col as usize));
                        }
                    }
                }
            }
        }

        if handled.len() == 100 {
            break;
        }

        step += 1;

        for h in handled {
            v[h.0][h.1] = 0;
        }

        //println!("After step {}:", i+1);
        //nice_print(&v);
    }

    step
}