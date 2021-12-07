use std::{fs::File, io::{BufReader, BufRead}};

pub fn run() -> (i64, i64) {
    a()
}

fn a() -> (i64, i64) {
    let f = BufReader::new(File::open("input/2015/day06.txt").unwrap());
    let mut brightnesses = [[0; 1000]; 1000];
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

        for i in y_min..y_max+1 {
            for j in x_min..x_max+1 {
                match action {
                    "toggle" => {
                        lights[i][j] = !lights[i][j];
                        brightnesses[i][j] += 2;
                    },
                    "turn on" => {
                        lights[i][j] = true;
                        brightnesses[i][j] += 1;
                    },
                    "turn off" => {
                        lights[i][j] = false;
                        if brightnesses[i][j] > 0 {
                            brightnesses[i][j] -= 1;
                        }
                    },
                    _ => {
                    },
                }
            }
        }
    }

    let mut count = 0;
    let mut brightness = 0;

    for i in 0..1000 {
        for j in 0..1000 {
            if lights[i][j] {
                count += 1;
            }
            brightness += brightnesses[i][j];
        }
    }

    (count, brightness)
}
