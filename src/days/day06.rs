pub fn run() -> (i64, i64) {
    (a(), b())
}

fn a() -> i64 {
    let mut input: Vec<i64> = std::fs::read_to_string("input/day06.txt").unwrap().split(',').map(|x| x.parse::<i64>().unwrap()).collect();
    let mut new_fish = Vec::<i64>::new();

    for _ in 0..80 {
        for fish in input.iter_mut() {
            if *fish == 0 {
                *fish = 6;
                new_fish.push(8);
            } else {
                *fish -= 1;
            }
        }

        input.append(&mut new_fish);
    }

    input.len() as i64
}

fn b() -> i64 {
    let input: Vec<i64> = std::fs::read_to_string("input/day06.txt").unwrap().split(',').map(|x| x.parse::<i64>().unwrap()).collect();

    // sum of fish for each "age" (days until multiplying)
    let mut count = [0; 9];

    // starting ages
    for i in input.iter() {
        count[(*i % 9) as usize] += 1;
    }

    // every day shift each fish down one day towards multiplying
    for _ in 0..256 {
        count.rotate_left(1);

        // copy the number of fish that just multiplied into the age 6 bucket
        count[6] += count[8];
    }

    count.iter().sum::<i64>()
}