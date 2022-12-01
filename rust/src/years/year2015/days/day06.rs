use std::{fs::File, io::{BufReader, BufRead}};

pub fn a() -> i64 {
    let f = BufReader::new(File::open("input/2015/day06.txt").unwrap());
    let mut lights = [[false; 1000]; 1000];

    for line in f.lines() {
        let real_line = line.unwrap();
        let mut rest = &real_line[..];
        let mut action = "";

        if let Some(suffix) = real_line.strip_prefix("toggle ") {
            rest = suffix;
            action = "toggle";
        } else if let Some(suffix) = real_line.strip_prefix("turn on ") {
            rest = suffix;
            action = "turn on";
        } else if let Some(suffix) = real_line.strip_prefix("turn off ") {
            rest = suffix;
            action = "turn off";
        }

        let mut str_points: Vec<&str> = rest.split(" through ").collect();
        let mut p2_v: Vec<usize> = str_points.pop().unwrap().split(',').map(|x| x.parse::<usize>().unwrap()).collect();
        let mut p1_v: Vec<usize> = str_points.pop().unwrap().split(',').map(|x| x.parse::<usize>().unwrap()).collect();

        let y1 = p1_v.pop().unwrap();
        let x1 = p1_v.pop().unwrap();
        let y2 = p2_v.pop().unwrap();
        let x2 = p2_v.pop().unwrap();

        let x_min = std::cmp::min(x1, x2);
        let x_max = std::cmp::max(x1, x2);
        let y_min = std::cmp::min(y1, y2);
        let y_max = std::cmp::max(y1, y2);

        for i in lights.iter_mut().take(y_max+1).skip(y_min) {
            for j in i.iter_mut().take(x_max+1).skip(x_min) {
                match action {
                    "toggle" => {
                        *j = !*j;
                    },
                    "turn on" => {
                        *j = true;
                    },
                    "turn off" => {
                        *j = false;
                    },
                    _ => {
                    },
                }
            }
        }
    }

    let mut count = 0;

    for i in &lights {
        for j in i {
            if *j {
                count += 1;
            }
        }
    }

    count
}

pub fn b() -> i64 {
    let f = BufReader::new(File::open("input/2015/day06.txt").unwrap());
    let mut brightnesses = [[0; 1000]; 1000];

    for line in f.lines() {
        let real_line = line.unwrap();
        let mut rest = &real_line[..];
        let mut action = "";

        if let Some(suffix) = real_line.strip_prefix("toggle ") {
            rest = suffix;
            action = "toggle";
        } else if let Some(suffix) = real_line.strip_prefix("turn on ") {
            rest = suffix;
            action = "turn on";
        } else if let Some(suffix) = real_line.strip_prefix("turn off ") {
            rest = suffix;
            action = "turn off";
        }

        let mut str_points: Vec<&str> = rest.split(" through ").collect();
        let mut p2_v: Vec<usize> = str_points.pop().unwrap().split(',').map(|x| x.parse::<usize>().unwrap()).collect();
        let mut p1_v: Vec<usize> = str_points.pop().unwrap().split(',').map(|x| x.parse::<usize>().unwrap()).collect();

        let y1 = p1_v.pop().unwrap();
        let x1 = p1_v.pop().unwrap();
        let y2 = p2_v.pop().unwrap();
        let x2 = p2_v.pop().unwrap();

        let x_min = std::cmp::min(x1, x2);
        let x_max = std::cmp::max(x1, x2);
        let y_min = std::cmp::min(y1, y2);
        let y_max = std::cmp::max(y1, y2);

        for i in brightnesses.iter_mut().take(y_max+1).skip(y_min) {
            for j in i.iter_mut().take(x_max+1).skip(x_min) {
                match action {
                    "toggle" => {
                        *j += 2;
                    },
                    "turn on" => {
                        *j += 1;
                    },
                    "turn off" => {
                        if *j > 0 {
                            *j -= 1;
                        }
                    },
                    _ => {
                    },
                }
            }
        }
    }

    let mut brightness = 0;

    for i in &brightnesses {
        for j in i {
            brightness += *j;
        }
    }

    brightness
}