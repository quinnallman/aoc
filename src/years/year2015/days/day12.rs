use serde_json::{Value};

pub fn a() -> i64 {
    let f: String = std::fs::read_to_string("input/2015/day12.txt").unwrap();
    let mut current = 0;
    let mut sum = 0;
    let mut parity = 1;
    for c in f.chars() {
        if c == '-' {
            parity = -1;
        } else if c.is_digit(10) {
            current *= 10;
            current += c as i64 - b'0' as i64;
        } else {
            sum += parity * current;
            current = 0;
            parity = 1;
        }
    }

    sum
}

fn get_sum(v: Value) -> i64 {
    match v {
        Value::Number(n) => {
            if n.is_i64() {
                n.as_i64().unwrap()
            } else {
                0
            }
        },
        Value::Array(a) => {
            a
                .iter()
                .fold(0, |acc, x| acc + get_sum(x.clone()))
        },
        Value::Object(o) => {
            let mut has_red = false;
            for (_, v) in o.clone() {
                if let Value::String(s) = v {
                    if s == "red" {
                        has_red = true;
                        break;
                    }
                }
            }
            if has_red {
                0
            } else {
                o
                    .iter()
                    .fold(0, |acc, x| acc + get_sum(x.1.clone()))
            }
        }
        _ => 0,
    }
}

pub fn b() -> i64 {
    let f: String = std::fs::read_to_string("input/2015/day12.txt").unwrap();
    let v: Value = serde_json::from_str(&f).unwrap();
    get_sum(v)
}