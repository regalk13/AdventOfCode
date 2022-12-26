#![feature(iter_array_chunks)]
#![feature(iter_intersperse)]

mod year2022;

use year2022::*;

pub trait Runit {
    fn parse(&mut self);
    fn first_part(&mut self) -> String;
    fn second_part(&mut self) -> String;
}

fn main() {
    println!("Hello, world!!");
    let mut day = year2022::day01::AocDay01::new();
    day.parse();
    println!("Output 1: {}", day.first_part());
    println!("Output 2: {}", day.second_part());
}
