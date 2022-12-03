#![feature(iter_array_chunks)]
use std::collections::{HashSet, HashMap};

fn convert_number(s: char) -> String {
    s.to_lowercase()
    .filter( |&letter| { letter as u8 >= 97 && letter as u8 <= 122 } )
    .map( |letter| { (letter as u8 - 96).to_string() } )
    .collect::<Vec<String>>()
    .join("")
}

fn first_part(file: String)  -> u32{
    file
    .lines()
    .flat_map(|line| {
        let (one, two) = line.split_at(line.len()/2);
        let one = one.chars().collect::<HashSet<_>>();
        return two
            .chars()
            .filter(move |c| one.contains(c))
            .collect::<HashSet::<char>>()
            .into_iter()
            .map(|c| { 
                let value: u32 = if c.is_ascii_lowercase() {
                        convert_number(c).parse::<u32>().unwrap()
                    } else {
                        convert_number(c).parse::<u32>().unwrap() + 26
                };
                return value
            })
    })
    .sum::<u32>()
}

fn second_part(file: String) -> u32 {

    file
    .lines()
    .array_chunks::<3>()
    .map(|[a,b,c]| {
        let chars = a
            .chars()
            .find(|a_char| {
                b.contains(*a_char)
                 && c.contains(*a_char)
            })
            .unwrap();
            let value: u32 = if chars.is_ascii_lowercase() {
                convert_number(chars).parse::<u32>().unwrap()
            } else {
                convert_number(chars).parse::<u32>().unwrap() + 26
            };
            return value
    })
    .sum::<u32>()
}

fn main() {
    let file = std::fs::read_to_string("./input")
    .unwrap();

    
    println!("Result: {}", second_part(file));
    //println!("Result: {}", first_part(file));
}
