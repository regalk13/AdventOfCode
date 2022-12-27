use crate::Runit;
use std::{cmp::Ordering, str::FromStr};

#[derive(Default)]
pub struct AocDay02 {
    input: String,
}

impl AocDay02 {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(PartialEq, Copy, Clone)]
enum Options {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for Options {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Options::Rock),
            "B" | "Y" => Ok(Options::Paper),
            "C" | "Z" => Ok(Options::Scissors),
            _ => Err("Invalid Move".to_string()),
        }
    }
}
impl PartialOrd for Options {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == &Options::Scissors && other == &Options::Rock {
            Some(Ordering::Less)
        } else if self == &Options::Rock && other == &Options::Scissors {
            Some(Ordering::Greater)
        } else {
            Some((*self as u8).cmp(&(*other as u8)))
        }
    }
}

impl Runit for AocDay02 {
    fn parse(&mut self) {
        self.input = crate::read_file("2022".to_string(), "02".to_string());
    }
    fn first_part(&mut self) -> String {
        let result: u32 = self
            .input
            .lines()
            .map(|line| {
                let moves: Vec<Options> = line
                    .split(" ")
                    .map(|s| s.parse::<Options>().unwrap())
                    .collect();
                match moves[0].partial_cmp(&moves[1]) {
                    Some(Ordering::Equal) => 3 + moves[1] as u32,
                    Some(Ordering::Less) => 6 + moves[1] as u32,
                    Some(Ordering::Greater) => 0 + moves[1] as u32,
                    None => {
                        panic!("moves should be comparable")
                    }
                }
            })
            .sum();

        result.to_string()
    }

    fn second_part(&mut self) -> String {
        let result: u32 = self
            .input
            .lines()
            .map(|line| {
                let moves: Vec<&str> = line.split(" ").collect();
                let opponent_move = moves[0].parse::<Options>().unwrap();
                match moves[1] {
                    "X" => {
                        let rmove = match opponent_move {
                            Options::Rock => Options::Scissors,
                            Options::Paper => Options::Rock,
                            Options::Scissors => Options::Paper,
                        };
                        0 + rmove as u32
                    }
                    "Y" => 3 + opponent_move as u32,
                    "Z" => {
                        let rmove = match opponent_move {
                            Options::Rock => Options::Paper,
                            Options::Paper => Options::Scissors,
                            Options::Scissors => Options::Rock,
                        };
                        6 + rmove as u32
                    }
                    _ => {
                        panic!("moves should be comparable")
                    }
                }
            })
            .sum();
        result.to_string()
    }
}
