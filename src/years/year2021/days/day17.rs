pub fn a() -> i64 {
    let f = std::fs::read_to_string("input/2021/day17.txt").unwrap();
    let mut gmax = i64::MIN;
    if let Some(rest) = f.strip_prefix("target area: ") {
        let coords: Vec<&str> = rest.split(", ").collect();
        if let Some(rest) = coords[0].strip_prefix("x=") {
            let x: Vec<i64> = rest.split("..").map(|x| x.parse::<i64>().unwrap()).collect();
            if let Some(rest) = coords[1].strip_prefix("y=") {
                let y: Vec<i64> = rest.split("..").map(|x| x.parse::<i64>().unwrap()).collect();

                for y_0 in -(x[1].abs())..x[1].abs() {
                    for x_0 in -(y[0].abs())..y[0].abs() {
                        let mut pos = (0, 0);
                        let mut speed = (x_0, y_0);
                        let mut max = i64::MIN;
                        loop {
                            pos.0 += speed.0;
                            pos.1 += speed.1;

                            if pos.1 > max {
                                max = pos.1;
                            }

                            if pos.0 >= x[0] && pos.0 <= x[1] && pos.1 >= y[0] && pos.1 <= y[1] {
                                if max > gmax {
                                    gmax = max;
                                }
                                break;
                            } else if pos.0 > x[1] || pos.1 < y[0] {
                                break;
                            }

                            speed.0 -= 1;
                            if speed.0 < 0 {
                                speed.0 = 0;
                            }
                            speed.1 -= 1;
                        }
                    }
                }
            }
        }
    }

    gmax
}

pub fn b() -> i64 {
    let f = std::fs::read_to_string("input/2021/day17.txt").unwrap();
    let mut solutions = Vec::new();
    if let Some(rest) = f.strip_prefix("target area: ") {
        let coords: Vec<&str> = rest.split(", ").collect();
        if let Some(rest) = coords[0].strip_prefix("x=") {
            let x: Vec<i64> = rest.split("..").map(|x| x.parse::<i64>().unwrap()).collect();
            if let Some(rest) = coords[1].strip_prefix("y=") {
                let y: Vec<i64> = rest.split("..").map(|x| x.parse::<i64>().unwrap()).collect();

                for y_0 in -(y[0].abs())..y[0].abs()+1 {
                    for x_0 in -(x[1].abs())..x[1].abs()+1 {
                        let mut pos = (0, 0);
                        let mut speed = (x_0, y_0);
                        loop {
                            pos.0 += speed.0;
                            pos.1 += speed.1;

                            if pos.0 >= x[0] && pos.0 <= x[1] && pos.1 >= y[0] && pos.1 <= y[1] {
                                solutions.push((x_0, y_0));
                                break;
                            } else if pos.0 > x[1] || pos.1 < y[0] {
                                break;
                            }

                            speed.0 -= 1;
                            if speed.0 < 0 {
                                speed.0 = 0;
                            }
                            speed.1 -= 1;
                        }
                    }
                }
            }
        }
    }

    solutions.len() as i64
}