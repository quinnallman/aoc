mod days;

use ansi_term::Colour::{Green, Red, Yellow};
use days::{day01, day02, day03, day04};
use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <day number>", args[0]);
        std::process::exit(1);
    }

    let days: Vec<i64> = args.iter().skip(1).filter_map(|day|
        day.parse::<i64>().ok()
    ).collect();
    
    for (i, day) in days.iter().enumerate() {
        let func = match day {
            1 => day01::run,
            2 => day02::run,
            3 => day03::run,
            4 => day04::run,
            _ => {
                println!("{}", Red.paint(format!("!!! I don't know what to do for day {} !!!", day)));
                break;
            },
        };

        println!("{}", Yellow.paint(format!("=== Day {:02} ===", day)));
        let start = Instant::now();
        let result = func();
        let duration = start.elapsed();
        println!("({}, {}) ({:.3}ms)", Green.paint(result.0.to_string()), Green.paint(result.1.to_string()), duration.as_millis());

        if i < days.len() - 1 {
            println!();
        }
    }
}