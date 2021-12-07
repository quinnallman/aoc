mod years;

use ansi_term::Colour::{Green, Red, Yellow};
//use years::year2015::days::{day01, day02, day03, day04, day05, day06, day07, day08};
use years::year2021::days::{day01, day02, day03, day04, day05, day06, day07};
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
            5 => day05::run,
            6 => day06::run,
            7 => day07::run,
            //8 => day08::run,
            _ => {
                println!("{}", Red.paint(format!("!!! I don't know what to do for day {} !!!", day)));
                break;
            },
        };

        println!("{}", Yellow.paint(format!("=== Day {:02} ===", day)));
        let start = Instant::now();
        let result = func();
        let duration = start.elapsed();
        println!("({}, {}) ({:.3}ms)", Green.paint(result.0.to_string()), Green.paint(result.1.to_string()), duration.as_secs_f32() * 1000f32);

        if i < days.len() - 1 {
            println!();
        }
    }
}