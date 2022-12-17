use std::str::FromStr;
use std::collections::HashMap;

#[derive(Debug, Default)]
struct Rocks {
    state: u32,
    positions: HashMap<(u32, u32), char>,
    jets: Vec<Jet>,
    jet_pos: usize,
    max: (u32, u32),
}

impl Rocks {

    fn trow_rocks(&mut self) {
        let mut rock = (self.max.0, self.max.1 + 3);
        if self.state == 1 {
            while let Some(next_pos) = self.fall(rock) {
                rock = next_pos;
            }
        }        
    }
    fn fall(&mut self, pos: (u32, u32)) -> Option<(u32, u32)> {
        let new_pos = (pos.0, pos.1 + 1);
        if !self.positions.contains_key(&new_pos) {
            
            if !self.positions.contains_key(&new_pons) {
                let new_pos = (if self.jets[self.jet_pos] == Jet::Left { pos.0 - 1 } else { pos.0 + 1 }, pos.1 + 1);
            }
            self.jet_pos += 1;
            return Some(new_pos);
        }
        None
    }
}


#[derive(Debug, PartialEq, Eq)]
enum Jet {
    Left,
    Right,
}

impl FromStr for Jet {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "<" => Ok(Jet::Left),
            ">" => Ok(Jet::Right),
            _ => Err("Invalid Move".to_string())
        }
    }

}

fn main() {
    let file = std::fs::read_to_string("./input.test").expect("Couldn't read input file");
    let mut jets: Vec<Jet> = vec![];
    for f in file.trim().chars().collect::<Vec<char>>() {
        jets.push(Jet::from_str(&f.to_string()).unwrap());
    }
    let mut rocks = Rocks::default();
    rocks.state = 1;
    rocks.jets = jets;
    rocks.max = (0, 0);
    rocks.jet_pos = 0;
    for _ in 1..=2022 {
        rocks.trow_rocks();
    }
}
