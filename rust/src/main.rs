mod years;

use std::env;
use std::time::Instant;
use ansi_term::Colour::{Green, Red, Yellow};

use years::year2015;
use years::year2021;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <day number>", args[0]);
        println!("    <day number> can include a year, or it will default to the current year");
        println!("    Examples:");
        println!("        {} 1", args[0]);
        println!("        {} 1 2 3 4 5", args[0]);
        println!("        {} 2015:1 2021:3 7", args[0]);
        std::process::exit(1);
    }

    for arg in args.iter().skip(1) {
        let mut day_vec: Vec<&str> = arg.split(':').collect();
        let mut year = 2021;
        let day = day_vec.pop().unwrap().parse::<i64>().unwrap();
        if !day_vec.is_empty() {
            year = day_vec.pop().unwrap().parse::<i64>().unwrap();
        }

        let func = match year {
            2015 => year2015::run,
            2021 => year2021::run,
            _ => {
                println!("{}", Red.paint(format!("!!! I don't know what to do for year {}!!!", year)));
                continue;
            },
        };

        println!("{}", Yellow.paint(format!("=== AOC {} Day {:02} ===", year, day)));
        let start = Instant::now();
        if let Some(result) = func(day) {
            let duration = start.elapsed();
            println!("({}, {}) ({:.3}ms)", Green.paint(result.0.to_string()), Green.paint(result.1.to_string()), duration.as_secs_f32() * 1000f32);
        } else {
            println!("{}", Red.paint(format!("!!! I don't know what to do for day {} in {}!!!", day, arg)));
        }
   }
}