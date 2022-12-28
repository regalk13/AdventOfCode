use crate::Runit;

use num_complex::Complex;
use std::collections::{HashMap, HashSet};

#[derive(Default, Clone)]
pub struct AocDay23 {
    elves_info: Elves,
}

impl AocDay23 {
    pub fn new() -> Self {
        AocDay23::default()
    }
}

// Elves, hashet of the map and the count for part 2
#[derive(Default, Clone)]
struct Elves {
    elves: HashSet<Complex<i32>>,
    count: i32,
}

impl Elves {
    // Builder of the srtuct
    fn new(input: &str) -> Self {
        const I_COM: Complex<i32> = Complex::new(0, 1);
        let inp_vec = input.trim().lines().collect::<Vec<&str>>();
        let mut elves: HashSet<Complex<i32>> = HashSet::new();
        // Parsing the input
        for (r, line) in inp_vec.iter().enumerate() {
            for (c, item) in line.chars().enumerate() {
                if item == '#' {
                    elves.insert(c as i32 + r as i32 * I_COM);
                }
            }
        }
        // Returning a new Elves
        Elves {
            elves: elves,
            count: 1,
        }
    }
    // Create the map scanner
    fn scan_map() -> HashMap<Complex<i32>, Vec<Complex<i32>>> {
        const I_COM: Complex<i32> = Complex::new(0, 1);
        let mut hash: HashMap<Complex<i32>, Vec<Complex<i32>>> = HashMap::new();
        // Scan each position need to consider of each move
        hash.insert(-I_COM, [-I_COM - 1, -I_COM, -I_COM + 1].to_vec());
        hash.insert(I_COM, [I_COM - 1, I_COM, I_COM + 1].to_vec());
        hash.insert(
            Complex::new(1, 0),
            [1 - I_COM, Complex::new(1, 0), 1 + I_COM].to_vec(),
        );
        hash.insert(
            Complex::new(-1, 0),
            [-1 - I_COM, Complex::new(-1, 0), -1 + I_COM].to_vec(),
        );
        hash
    }

    // Move the elft in a loop until find the last possible position
    fn move_elves_part2(&mut self) {
        // Creathe i <- sqrt(-1)
        const I_COM: Complex<i32> = Complex::new(0, 1);
        // Get the map to scan
        let scan_map = Self::scan_map();
        // Possible  moves
        let mut moves = [-I_COM, I_COM, Complex::new(-1, 0), Complex::new(1, 0)].to_vec();
        let n = [
            -1 - I_COM,
            -I_COM,
            -I_COM + 1,
            Complex::new(1, 0),
            1 + I_COM,
            I_COM,
            I_COM - 1,
            Complex::new(-1, 0),
        ];
        // The las map is the current map until it changes
        let mut last_map = self.elves.clone();
        loop {
            // Position considered just one time
            let mut once = HashSet::new();
            // Position considered two times
            let mut twice = HashSet::new();
            // Iterate the positios
            for elf in &self.elves {
                // Check if there is a elve in the position to consider
                if n.iter().all(|&x| !(self.elves.contains(&(elf + x)))) {
                    continue;
                }
                // Consider moves
                for move_ in &moves {
                    // consider the three points each move
                    if scan_map[move_]
                        .iter()
                        .all(|&x| !(self.elves.contains(&(elf + x))))
                    {
                        let prop = elf + move_;
                        // Save twice or once
                        if twice.contains(&prop) {
                            // nothing
                        } else if once.contains(&prop) {
                            twice.insert(prop);
                        } else {
                            once.insert(prop);
                        }
                        break;
                    }
                }
            }
            // Second part of the round
            let ec = self.elves.clone();

            for elf in &ec {
                if n.iter().all(|&x| !(ec.contains(&(elf + x)))) {
                    continue;
                }
                // Check for moves considered once and change the position
                for move_ in &moves {
                    if scan_map[move_].iter().all(|&x| !(ec.contains(&(elf + x)))) {
                        let prop = elf + move_;
                        if !(twice.contains(&prop)) {
                            self.elves.remove(elf);
                            self.elves.insert(prop);
                        }
                        break;
                    }
                }
            }
            // Change moves
            let moved = moves.remove(0);
            moves.push(moved);
            // Check if the last map is the same to the current, if its you have the count
            if last_map == self.elves {
                break;
            }

            last_map = self.elves.clone();

            self.count += 1;
        }
    }

    fn move_elves(&mut self) {
        const I_COM: Complex<i32> = Complex::new(0, 1);
        let scan_map = Self::scan_map();
        let mut moves = [-I_COM, I_COM, Complex::new(-1, 0), Complex::new(1, 0)].to_vec();
        let n = [
            -1 - I_COM,
            -I_COM,
            -I_COM + 1,
            Complex::new(1, 0),
            1 + I_COM,
            I_COM,
            I_COM - 1,
            Complex::new(-1, 0),
        ];
        for _ in 0..10 {
            let mut once = HashSet::new();
            let mut twice = HashSet::new();
            for elf in &self.elves {
                if n.iter().all(|&x| !(self.elves.contains(&(elf + x)))) {
                    continue;
                }

                for move_ in &moves {
                    if scan_map[move_]
                        .iter()
                        .all(|&x| !(self.elves.contains(&(elf + x))))
                    {
                        let prop = elf + move_;
                        if twice.contains(&prop) {
                            // nothing
                        } else if once.contains(&prop) {
                            twice.insert(prop);
                        } else {
                            once.insert(prop);
                        }
                        break;
                    }
                }
            }

            let ec = self.elves.clone();

            for elf in &ec {
                if n.iter().all(|&x| !(ec.contains(&(elf + x)))) {
                    continue;
                }

                for move_ in &moves {
                    if scan_map[move_].iter().all(|&x| !(ec.contains(&(elf + x)))) {
                        let prop = elf + move_;
                        if !(twice.contains(&prop)) {
                            self.elves.remove(elf);
                            self.elves.insert(prop);
                        }
                        break;
                    }
                }
            }

            let moved = moves.remove(0);
            moves.push(moved);
        }
    }
}

impl Runit for AocDay23 {
    fn parse(&mut self) {
        self.elves_info = Elves::new(&crate::read_file("2022".to_string(), "23".to_string()));
    }

    fn second_part(&mut self) -> String {
        let mut elves_info = self.elves_info.clone();
        elves_info.move_elves_part2();
        // Return the count of iterations
        elves_info.count.to_string()
    }

    fn first_part(&mut self) -> String {
        let mut elves_info = self.elves_info.clone();

        elves_info.move_elves();
        // using the complex numbers: x - real part, y - imaginary part
        // Min value of x, max value of x, min value of y and max value of y
        // this gives you the size of the rectangle where the elves are
        let minx = elves_info.elves.iter().map(|x| x.re).min();
        let maxx = elves_info.elves.iter().map(|x| x.re).max();
        let miny = elves_info.elves.iter().map(|x| x.im).min();
        let maxy = elves_info.elves.iter().map(|x| x.im).max();
        // Return the empty spaces
        ((maxx.unwrap() - minx.unwrap() + 1) * (maxy.unwrap() - miny.unwrap() + 1)
            - elves_info.elves.len() as i32)
            .to_string()
    }
}
