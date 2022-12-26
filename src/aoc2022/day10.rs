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
/*
let mut ctr_pixels: String = "".to_string();
let mut position = 0;
for f in file.trim().split("\n") {
    let instruction  = f.split_once(" ");

    if ctr.cycle == 39 {
        ctr.draw_row();
        ctr.clear_positions();
        position = 0;
    }

    if ctr.cycle == 80 {
        ctr.draw_row();
        ctr.clear_positions();
        position = 0;
    }

    if ctr.cycle == 120 {
        ctr.draw_row();
        ctr.clear_positions();
        position = 0;
    }

    if ctr.cycle == 160 {
        ctr.draw_row();
        ctr.clear_positions();
        position = 0;
    }

    if ctr.cycle == 200 {
        ctr.draw_row();
        ctr.clear_positions();
        position = 0;
    }
    if ctr.cycle == 239 {
        ctr.draw_row();
        ctr.clear_positions();
        position = 0;
    }

    ctr.cycle += 1;

    if instruction != None {
        let (_, value) = instruction.unwrap();
    //    if
        if ((ctr.x-1)..=(ctr.x+1)).contains(&(position)) {

            println!("{}:{} #", position+1, ctr.x);
            ctr.add_position(position+1);
        } else {
            println!("{}:{} .", position+1, ctr.x);
        }
        ctr.cycle += 1;
        position += 2;
        ctr.add_cycle(value.parse::<i32>().unwrap());
        if ((ctr.x-1)..=(ctr.x+1)).contains(&(position)) {
             println!("{}:{} #", position+1, ctr.x);
            ctr.add_position(position);
        } else {
            println!("{}:{} .", position+1, ctr.x);
        }
        //if ((ctr.x -1)..=(x+1)).contains(&(position+1)) {
        //}
        //     println!("Adding positions: {}", ctr.x);
    } else {
        position += 1;
    }
};

// println!("Cpu X: {}, cpu cycles: {}", cpu.x, cpu.cycle);*/

fn second_part(file: &str) -> String {
    let (_, instructions) = instruction_set(file).unwrap();
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

fn first_part(file: String) -> i32 {
    let mut cpu = CPU { x: 1, cycle: 0 };

    let mut signal_strength = vec![];
    for f in file.trim().split("\n") {
        let instruction = f.split_once(" ");
        cpu.cycle += 1;
        if cpu.cycle == 20
            || cpu.cycle == 60
            || cpu.cycle == 100
            || cpu.cycle == 140
            || cpu.cycle == 180
            || cpu.cycle == 220
        {
            println!("Signal: {}, cpu cycle: {}", cpu.x * cpu.cycle, cpu.cycle);
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
                println!("Signal: {}, cpu cycle: {}", cpu.x * cpu.cycle, cpu.cycle);
                signal_strength.push(cpu.x * cpu.cycle);
            }
            cpu.add_cycle(value.parse::<i32>().unwrap());
        }
    }

    println!("Cpu X: {}, cpu cycles: {}", cpu.x, cpu.cycle);
    signal_strength.iter().sum::<i32>()
}

fn main() {
    let file = std::fs::read_to_string("./input-2.test").expect("Couldn't read input file");
    println!("Sum: {}", first_part(file));
    // println!("{}", second_part(&file));
}
