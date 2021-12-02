mod days;

use days::day01;
use ansi_term::Colour::{Green, Red, Yellow};
use std::env;

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
            _ => {
                println!("{}", Red.paint(format!("!!! I don't know what to do for day {} !!!", day)));
                break;
            },
        };

        println!("{}", Yellow.paint(format!("=== Day {:02} ===", day)));
        let result = func();

        if i < days.len() - 1 {
            println!();
        }
    }
}