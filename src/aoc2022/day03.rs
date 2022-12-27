use crate::Runit;
use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct AocDay03 {
    input: String,
}
impl AocDay03 {
    pub fn new() -> Self {
        Self::default()
    }
}

fn convert_number(s: char) -> String {
    s.to_lowercase()
        .filter(|&letter| letter as u8 >= 97 && letter as u8 <= 122)
        .map(|letter| (letter as u8 - 96).to_string())
        .collect::<Vec<String>>()
        .join("")
}

impl Runit for AocDay03 {
    fn parse(&mut self) {
        self.input = crate::read_file("2022".to_string(), "03".to_string());
    }

    fn first_part(&mut self) -> String {
        self.input
            .lines()
            .flat_map(|line| {
                let (one, two) = line.split_at(line.len() / 2);
                let one = one.chars().collect::<HashSet<_>>();
                return two
                    .chars()
                    .filter(move |c| one.contains(c))
                    .collect::<HashSet<char>>()
                    .into_iter()
                    .map(|c| {
                        let value: u32 = if c.is_ascii_lowercase() {
                            convert_number(c).parse::<u32>().unwrap()
                        } else {
                            convert_number(c).parse::<u32>().unwrap() + 26
                        };
                        return value;
                    });
            })
            .sum::<u32>()
            .to_string()
    }

    fn second_part(&mut self) -> String {
        self.input
            .lines()
            .array_chunks::<3>()
            .map(|[a, b, c]| {
                let chars = a
                    .chars()
                    .find(|a_char| b.contains(*a_char) && c.contains(*a_char))
                    .unwrap();
                let value: u32 = if chars.is_ascii_lowercase() {
                    convert_number(chars).parse::<u32>().unwrap()
                } else {
                    convert_number(chars).parse::<u32>().unwrap() + 26
                };
                return value;
            })
            .sum::<u32>()
            .to_string()
    }
}
