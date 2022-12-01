use std::{fs::File, io::BufReader, io::BufRead};

pub fn a() -> i64 {
    let f = BufReader::new(File::open("input/2021/day05.txt").unwrap());
    let mut map = [[0usize; 1000]; 1000];

    for line in f.lines() {
        let mut lines: Vec<String> = line.unwrap().split(" -> ").map(String::from).collect();
        let mut p2: Vec<usize> = lines.pop().unwrap().split(',').map(|x| x.parse::<usize>().unwrap()).collect();
        let mut p1: Vec<usize> = lines.pop().unwrap().split(',').map(|x| x.parse::<usize>().unwrap()).collect();

        let y1 = p1.pop().unwrap();
        let x1 = p1.pop().unwrap();
        let y2 = p2.pop().unwrap();
        let x2 = p2.pop().unwrap();

        if x1 == x2 {
            let y_low = std::cmp::min(y1, y2);
            let y_high = std::cmp::max(y1, y2);
            for row in map.iter_mut().take(y_high+1).skip(y_low) {
                row[x1] += 1;
            }
        } else if y1 == y2 {
            let x_low = std::cmp::min(x1, x2);
            let x_high = std::cmp::max(x1, x2);
            for i in x_low..x_high+1 {
                map[y1][i] += 1;
            }
        }
    }

    let mut count = 0;
    for row in &map {
        for cell in row {
            if *cell > 1 {
                count += 1;
            }
        }
    }

    count
}

pub fn b() -> i64 {
    let f = BufReader::new(File::open("input/2021/day05.txt").unwrap());
    let mut map = [[0usize; 1000]; 1000];

    for line in f.lines() {
        let real_line = line.unwrap();
        let mut lines: Vec<String> = real_line.split(" -> ").map(String::from).collect();
        let mut p2: Vec<usize> = lines.pop().unwrap().split(',').map(|x| x.parse::<usize>().unwrap()).collect();
        let mut p1: Vec<usize> = lines.pop().unwrap().split(',').map(|x| x.parse::<usize>().unwrap()).collect();

        let y1 = p1.pop().unwrap();
        let x1 = p1.pop().unwrap();
        let y2 = p2.pop().unwrap();
        let x2 = p2.pop().unwrap();

        let x_low = std::cmp::min(x1, x2);
        let x_high = std::cmp::max(x1, x2);
        let y_low = std::cmp::min(y1, y2);
        let y_high = std::cmp::max(y1, y2);

        if x1 == x2 {
            //for i in y_low..y_high+1 {
            for row in map.iter_mut().take(y_high+1).skip(y_low) {
                row[x1] += 1;
            }
        } else if y1 == y2 {
            for i in x_low..x_high+1 {
                map[y1][i] += 1;
            }
        } else {
            let dist = y_high - y_low;
            let mut x_skip: isize = 1;
            let mut y_skip: isize = 1;
            if y1 > y2 {
                y_skip = -1;
            }
            if x1 > x2 {
                x_skip = -1;
            }
            for i in 0..dist + 1 {
                let x = (x1 as isize + i as isize * x_skip) as usize;
                let y = (y1 as isize + i as isize * y_skip) as usize;
                map[y][x] += 1;
            }
        }
    }

    let mut count = 0;
    for row in &map {
        for cell in row {
            if *cell > 1 {
                count += 1;
            }
        }
    }

    count
}
