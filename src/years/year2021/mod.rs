pub mod days;

pub fn run(day: i64) -> Option<(i64, i64)> {
    match day {
        1 => Some((days::day01::a(), days::day01::b())),
        2 => Some((days::day02::a(), days::day02::b())),
        3 => Some((days::day03::a(), days::day03::b())),
        4 => Some((days::day04::a(), days::day04::b())),
        5 => Some((days::day05::a(), days::day05::b())),
        6 => Some((days::day06::a(), days::day06::b())),
        7 => Some((days::day07::a(), days::day07::b())),
        8 => Some((days::day08::a(), days::day08::b())),
        9 => Some((days::day09::a(), days::day09::b())),
        10 => Some((days::day10::a(), days::day10::b())),
        _ => {
            None
        }
    }
}