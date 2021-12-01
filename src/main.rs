mod days;

use ansi_term::Colour;
use days::day01;
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
            1 => Some(day01::run),
            _ => None,
        };

        if let Some(func) = func {
            println!("{}", Colour::Yellow.paint(format!("=== Day {:02} ===", day)));
            func();
        } else {
            println!("{}", Colour::Red.paint(format!("!!! I don't know what to do for day {} !!!", day)));
        }

        if i < days.len() - 1 {
            println!();
        }
    }
}