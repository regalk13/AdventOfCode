use crate::Runit;

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    character::complete::{self, alpha1, digit1, newline},
    combinator::verify,
    multi::{many1, separated_list1},
    sequence::{preceded, separated_pair},
    *,
};
use std::collections::{BTreeMap, HashSet};

#[derive(Default)]
pub struct AocDay10 {
    file: String,
}

impl AocDay10 {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug)]
enum Instruction {
    Noop,
    Add(i32),
}

use Instruction::*;

impl Instruction {
    fn cycles(&self) -> u32 {
        match self {
            Noop => 1,
            Add(_) => 2,
        }
    }
}

fn instruction_set(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, vecs) = separated_list1(
        newline,
        alt((
            tag("noop").map(|_| Noop),
            preceded(tag("addx "), complete::i32).map(|num| Add(num)),
        )),
    )(input)?;

    Ok((input, vecs))
}

// Cpu
struct CPU {
    x: i32,
    cycle: i32,
}

impl CPU {
    fn add_cycle(&mut self, value: i32) {
        self.x += value;
    }
}

// Crash team racing
struct CTR {
    x: i32,
    cycle: i32,
    positions: HashSet<i32>,
}

impl CTR {
    fn add_cycle(&mut self, value: i32) {
        self.x += value;
    }

    fn clear_positions(&mut self) {
        self.positions.clear();
    }

    fn add_position(&mut self, position: i32) {
        self.positions.insert(position);
    }

    fn draw_row(&mut self) {
        for i in 0..40 {
            if self.positions.contains(&i) || i == 0 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

impl Runit for AocDay10 {
    fn parse(&mut self) {
        self.file = crate::read_file("2022".to_string(), "10".to_string());
    }
    fn second_part(&mut self) -> String {
        let (_, instructions) = instruction_set(&self.file).unwrap();
        let mut x: i32 = 1;
        let mut cycles: u32 = 0;
        let mut crt_pixels: String = "".to_string();

        for instruction in instructions.iter() {
            for cycle_add in 0..instruction.cycles() {
                let pixel_id = (cycles as i32 + cycle_add as i32) % 40;

                if ((x - 1)..=(x + 1)).contains(&pixel_id) {
                    crt_pixels.push_str("#");
                } else {
                    crt_pixels.push_str(".");
                }
            }

            cycles += instruction.cycles();
            match instruction {
                Noop => {}
                Add(num) => {
                    x += num;
                }
            };
        }

        crt_pixels
            .chars()
            .chunks(40)
            .into_iter()
            .map(|chunk| chunk.collect::<String>())
            .join("\n")
    }

    fn first_part(&mut self) -> String {
        let mut cpu = CPU { x: 1, cycle: 0 };

        let mut signal_strength = vec![];
        for f in self.file.trim().split("\n") {
            let instruction = f.split_once(" ");
            cpu.cycle += 1;
            if cpu.cycle == 20
                || cpu.cycle == 60
                || cpu.cycle == 100
                || cpu.cycle == 140
                || cpu.cycle == 180
                || cpu.cycle == 220
            {
                signal_strength.push(cpu.x * cpu.cycle);
            }

            if instruction != None {
                let (_, value) = instruction.unwrap();
                cpu.cycle += 1;
                if cpu.cycle == 20
                    || cpu.cycle == 60
                    || cpu.cycle == 100
                    || cpu.cycle == 140
                    || cpu.cycle == 180
                    || cpu.cycle == 220
                {
                    signal_strength.push(cpu.x * cpu.cycle);
                }
                cpu.add_cycle(value.parse::<i32>().unwrap());
            }
        }

        signal_strength.iter().sum::<i32>().to_string()
    }
}
