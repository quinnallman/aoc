use itertools::Itertools;
use std::cmp::Ordering;

fn is_valid(s: &str) -> bool {
    if s.contains('i') || s.contains('o') || s.contains('l') {
        return false;
    }

    let mut has_seq = false;
    for (a, b, c) in s.chars().tuple_windows() {
        if b as u32 == a as u32 + 1 && c as u32 == b as u32 + 1 {
            has_seq = true;
            break;
        }
    }

    let mut has_doubles = false;
    let mut num_doubles = 0;
    let mut doubles_chars = Vec::new();
    let mut doubles_indices = Vec::new();
    for (i, (a, b)) in s.chars().tuple_windows().enumerate() {
        if a == b {
            num_doubles += 1;
            doubles_chars.push(a);
            doubles_indices.push(i);
        }
    }

    match num_doubles.cmp(&2) {
        Ordering::Greater => {
            has_doubles = true;
        },
        Ordering::Equal => {
            let c1 = doubles_chars.pop().unwrap();
            let c2 = doubles_chars.pop().unwrap();
    
            if c1 != c2 {
                has_doubles = true;
            } else {
                let i1 = doubles_indices.pop().unwrap();
                let i2 = doubles_indices.pop().unwrap();
    
                if ((i1 - i2) as i64).abs() > 1 {
                    has_doubles = true;
                }
            }
            },
        _ => {
        },
    }

    has_seq && has_doubles
}

pub fn a() -> i64 {
    let mut input = vec!['h', 'e', 'p', 'x', 'c', 'r', 'r', 'q'];

    loop {
        while input.len() < 8 {
            input.push('a');
        }

        let i_str = &input.iter().collect::<String>()[..];
        //println!("{}", i_str);
        if is_valid(i_str) {
            break;
        }

        let mut c = input.pop().unwrap();
        while c == 'z' {
            c = input.pop().unwrap();
        }
        if c == 'h' || c == 'k' || c == 'n' {
            c = (c as u8 + 2) as char;
        } else {
            c = (c as u8 + 1) as char;
        }
        input.push(c);
    }

    println!("{:?}", input.into_iter().collect::<String>());

    0
}

pub fn b() -> i64 {
    let mut input = vec!['h', 'e', 'p', 'x', 'x', 'z', 'a', 'a'];

    loop {
        while input.len() < 8 {
            input.push('a');
        }

        let i_str = &input.iter().collect::<String>()[..];
        //println!("{}", i_str);
        if is_valid(i_str) {
            break;
        }

        let mut c = input.pop().unwrap();
        while c == 'z' {
            c = input.pop().unwrap();
        }
        if c == 'h' || c == 'k' || c == 'n' {
            c = (c as u8 + 2) as char;
        } else {
            c = (c as u8 + 1) as char;
        }
        input.push(c);
    }

    println!("{:?}", input.into_iter().collect::<String>());

    0
}