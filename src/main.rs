#![feature(iter_array_chunks)]
#![feature(iter_intersperse)]
use std::time::{Duration, Instant};
mod aoc2022;
mod aoc2023;
mod aoc2024;

use aoc2022::*;
use aoc2023::*;
use aoc2024::*;

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

    let years = vec![run_2022, run_2023, run_2024];
    if let Ok(year) = args[1].parse::<u32>() {
        if (2022..=2024).contains(&year) {
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
    let start = Instant::now();
    day.parse();
    let parse_time = start.elapsed().as_millis();
    println!(
        "Time parsing: {:3}.{:03}",
        parse_time / 1000,
        parse_time % 1000
    );
    println!();
    println!("Part 1");
    let start = Instant::now();
    let part_1 = day.first_part();
    let part1_time = start.elapsed().as_millis();
    println!(
        "Time running: {:3}.{:03}",
        part1_time / 1000,
        part1_time % 1000
    );
    println!("Result: {}", part_1);

    println!();
    println!("Part 2");
    let start = Instant::now();
    let part_2 = day.second_part();
    let part2_time = start.elapsed().as_millis();
    println!(
        "Time running: {:3}.{:03}",
        part2_time / 1000,
        part2_time % 1000
    );
    println!("Result: {}", part_2);
}
