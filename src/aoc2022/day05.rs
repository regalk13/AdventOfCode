use crate::Runit;

// Use HashMap data structure
use std::collections::HashMap;

#[derive(Default)]
pub struct AocDay05 {
    hash: HashMap<u32, Vec<char>>,
    moves: Vec<u32>,
}

impl AocDay05 {
    pub fn new() -> Self {
        Self::default()
    }
}

fn permut(
    mut hash: HashMap<u32, Vec<char>>,
    mut moves: Vec<u32>,
    part2: bool,
) -> HashMap<u32, Vec<char>> {
    // Takes 3 moves at the time
    while moves.len() >= 3 {
        // row source
        let content = hash[&moves[1]]
            .iter()
            .take(moves[0] as usize)
            .collect::<Vec<&char>>();
        // saving row source
        let mut content2: Vec<char> = vec![];
        for c in content {
            if part2 {
                content2.push(*c);
            // part 2
            } else {
                content2.insert(0, *c);
            }
        }
        // Getting source
        for c in content2 {
            // Moving crates to the new row (vector)
            hash.entry(moves[2]).or_insert(Vec::new()).insert(0, c);
        }

        // Remove crates from row source
        hash.get_mut(&moves[1])
            .expect("expected a item lol")
            .drain(0..moves[0] as usize);

        // Remove moves used!
        moves.remove(0);
        moves.remove(0);
        moves.remove(0);
    }
    // Return hash permuted
    hash
}

impl Runit for AocDay05 {
    fn parse(&mut self) {
        // Reading the file ./input.test for testing!
        let file = crate::read_file("2022".to_string(), "05".to_string());
        // Creating hashMap
        let mut cargo_hash: HashMap<u32, Vec<char>> = HashMap::new();

        // First input formating. [0] -> crates, [1] -> moves
        let input = file.split("\n\n").collect::<Vec<&str>>();

        // Crates
        let _cargo_crane = input[0]
            .split("\n")
            .map(|line| {
                let chars = line
                    // Cleaning crates
                    .replace(&['[', ']', ',', '\"', '.', ';', ':', '\''][..], "")
                    // Creating vector of chars
                    .chars()
                    // Collect it
                    .collect::<Vec<char>>();

                // Variable of moving
                let mut i: u32 = 0;
                // Counting spaces, if 4s == space
                let mut s: u32 = 0;
                let mut space: u32 = 0;
                // Loopin in chars
                for c in chars {
                    // Checking is it's ar white space or not
                    if !(c.is_alphabetic()) {
                        // Adding 1 for spacing
                        s += 1;
                        if s >= 4 {
                            s = 0;
                            space += 1;
                        }
                        continue;
                    } else {
                        s = 0;
                        i += 1;
                        // Create entry to the hashmap, spaces+i index
                        cargo_hash.entry(space + i).or_insert(Vec::new()).push(c);
                    }
                }
                // return the line
                line
            })
            // Collecting in a vector
            .collect::<Vec<&str>>();
        // Moves
        let moves = input[1].trim().split("\n").collect::<Vec<&str>>();

        // Vector of u32
        let mut moves_numbers: Vec<u32> = vec![];

        // Looping and adding 3 moves at the time in order
        for f in moves {
            // Line split removing works like from, to ....
            let spliter = f.split(" ").collect::<Vec<&str>>();

            moves_numbers.push(spliter[1].parse::<u32>().expect("Valid"));
            moves_numbers.push(spliter[3].parse::<u32>().expect("Valid"));
            moves_numbers.push(spliter[5].parse::<u32>().expect("Valid"));
        }
        self.hash = cargo_hash;
        self.moves = moves_numbers;
    }
    // Function that permut the crates in the cargo
    fn first_part(&mut self) -> String {
        // Getting the hash permuted
        let hash_permuted = permut(self.hash.clone(), self.moves.clone(), false);

        let mut output = "".to_string();
        // Printing the crates that are in the top
        for f in 0..hash_permuted.len() as u32 {
            output += &hash_permuted[&(f + 1)][0].to_string();
        }
        output
    } // Function that permut the crates in the cargo
    fn second_part(&mut self) -> String {
        // Getting the hash permuted
        let hash_permuted = permut(self.hash.clone(), self.moves.clone(), true);
        let mut output = "".to_string();
        // Printing the crates that are in the top
        for f in 0..hash_permuted.len() as u32 {
            output += &hash_permuted[&(f + 1)][0].to_string();
        }
        output
    }
}
