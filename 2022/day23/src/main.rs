use num_complex::Complex;
use std::collections::{HashMap, HashSet};

struct Elves {
    elves: HashSet<Complex<i32>>,
    count: i32,
}

impl Elves {
    fn new(input: &str) -> Self {
        const I_COM: Complex<i32> = Complex::new(0, 1);
        let inp_vec = input.trim().lines().collect::<Vec<&str>>();
        let mut elves: HashSet<Complex<i32>> = HashSet::new();
        for (r, line) in inp_vec.iter().enumerate() {
            for (c, item) in line.chars().collect::<Vec<char>>().iter().enumerate() {
                if *item == '#' {
                    elves.insert(c as i32 + r as i32 * I_COM);
                }
            }
        }
        Elves {
            elves: elves,
            count: 1,
        }
    }

    fn scan_map() -> HashMap<Complex<i32>, Vec<Complex<i32>>> {
        const I_COM: Complex<i32> = Complex::new(0, 1);
        let mut hash: HashMap<Complex<i32>, Vec<Complex<i32>>> = HashMap::new();
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

    fn move_elves_part2(&mut self) {
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
        let mut last_map = self.elves.clone();
        while true {
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

fn second_part(file: &str) -> i32 {
    let mut elves_info = Elves::new(file);
    elves_info.move_elves_part2();
    elves_info.count
}

fn first_part(file: &str) -> i32 {
    let mut elves_info = Elves::new(file);
    elves_info.move_elves();

    let minx = elves_info.elves.iter().map(|x| x.re).min();
    let maxx = elves_info.elves.iter().map(|x| x.re).max();
    let miny = elves_info.elves.iter().map(|x| x.im).min();
    let maxy = elves_info.elves.iter().map(|x| x.im).max();

    (maxx.unwrap() - minx.unwrap() + 1) * (maxy.unwrap() - miny.unwrap() + 1)
        - elves_info.elves.len() as i32
}

fn main() {
    let file = std::fs::read_to_string("./input").expect("Expected file");

    println!("Output 1: {}", first_part(&file));
    println!("Output 1: {}", second_part(&file));
}
