pub fn a() -> i64 {
    const MAX_ITERATIONS: usize = 40;

    let mut input = String::from("3113322113");
    let mut result = String::new();

    for _ in 0..MAX_ITERATIONS {
        let mut output = String::new();
        let mut count = 0;
        let mut old_c = '\0';
        for c in input.chars() {
            if c != old_c  && old_c != '\0' {
                output.push_str(&count.to_string()[..]);
                output.push(old_c);
                count = 1;
            } else {
                count += 1;
            }

            old_c = c;
        }

        output.push_str(&count.to_string()[..]);
        output.push(old_c);

        input = output.clone();
        result = output.clone();
    }

    result.len() as i64
}

pub fn b() -> i64 {
    const MAX_ITERATIONS: usize = 50;

    let mut input = String::from("3113322113");
    let mut result = String::new();

    for _ in 0..MAX_ITERATIONS {
        let mut output = String::new();
        let mut count = 0;
        let mut old_c = '\0';
        for c in input.chars() {
            if c != old_c  && old_c != '\0' {
                output.push_str(&count.to_string()[..]);
                output.push(old_c);
                count = 1;
            } else {
                count += 1;
            }

            old_c = c;
        }

        output.push_str(&count.to_string()[..]);
        output.push(old_c);

        input = output.clone();
        result = output.clone();
    }

    result.len() as i64
}