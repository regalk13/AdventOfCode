#![feature(iter_array_chunks)]
#![feature(iter_intersperse)]

mod aoc2022;

use aoc2022::*;

pub trait Runit {
    fn parse(&mut self);
    fn first_part(&mut self) -> String;
    fn second_part(&mut self) -> String;
}

pub fn read_file(year: String, day: String) -> String {
    let template = format!("input/{}/input_day{}", year, day);
    std::fs::read_to_string(template).expect("Couldn't read file")
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 3 {
        println!("Invalid usage");
        println!("Example of use: cargo run [year] [day]");
        std::process::exit(1);
    }

    let years = vec![run_2022];
    if let Ok(year) = args[1].parse::<u32>() {
        if (2015..=2022).contains(&year) {
            if let Ok(day_) = args[2].parse::<u32>() {
                println!("Running day {}, from {}", day_, year);
                years[year as usize - 2022](day_)
            } else {
                println!("Invalid day: {:?}", args[2])
            }
        } else {
            println!("Invalid year, valid format example: 2022");
        }
    } else {
        println!("Invalid year, valid format example: 2022");
    }
}

pub fn run<T: Runit + ?Sized>(day: &mut T) {
    day.parse();

    println!("Part 1 {}", day.first_part());
    println!("Part 2 {}", day.second_part());
}
