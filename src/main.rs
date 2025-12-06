use functions::FUNCTIONS;
use std::env;
use std::time::{self};

mod chargrid;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod functions;
mod inputs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: {} <day> <part> <test>", args[0]);
        std::process::exit(1);
    }

    let day = &args[1];
    let part = &args[2];
    let test = args[3].parse::<bool>().unwrap_or(false);

    let now = time::Instant::now();
    let result = match FUNCTIONS.get(&(day.as_str(), part.as_str())) {
        Some(func) => func(test)?,
        None => {
            eprintln!("Invalid day or part");
            std::process::exit(1);
        }
    };
    let delta = now.elapsed();
    if delta.as_millis() <= 10 {
        println!(
            "Result : {} // Time elapsed : {}µs",
            result,
            now.elapsed().as_micros()
        );
    } else if delta.as_millis() >= 10000 {
        println!(
            "Result : {} // Time elapsed : {}s",
            result,
            now.elapsed().as_secs()
        );
    } else {
        println!(
            "Result : {} // Time elapsed : {}ms",
            result,
            now.elapsed().as_millis()
        );
    }

    Ok(())
}
