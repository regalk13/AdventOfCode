use std::collections::HashMap;


fn permut(mut hash: HashMap<u32, Vec<char>>, mut moves: Vec<u32>) -> HashMap<u32, Vec<char>> {
   
    while moves.len() >= 3 {
        let content: Vec<&char>;
        
        content = hash[&moves[1]].iter().take(moves[0] as usize).collect::<Vec<&char>>();
        let mut content2: Vec<char> = vec![];
        for c in content {
            //part 1
            content2.push(*c);
            // part 2
            //content2.insert(0, *c); 
        }
        
        for c in content2 {
            hash.entry(moves[2]).or_insert(Vec::new()).insert(0, c); 
        }
        

        hash.get_mut(&moves[1]).expect("expected a item lol").drain(0..moves[0] as usize);

        moves.remove(0);
        moves.remove(0);
        moves.remove(0);
    }
    hash  
}

fn first_part(file: String) -> u32 { 
    let mut cargo_hash: HashMap<u32, Vec<char> > = HashMap::new(); 
    let input = file
    .split("\n\n")
    .collect::<Vec<&str>>();
    

    let _cargo_crane = input[0]
    .split("\n")
    .map(|line| {
        let chars = line
            .replace(&['[', ']', ',', '\"', '.', ';', ':', '\''][..], "")
            .chars()
            .collect::<Vec<char>>(); 
        let mut i: u32 = 0;
            let mut s: u32 = 0;
        let mut space: u32 = 0;
        for c in chars {
            if !(c.is_alphabetic()) {
                s += 1;
                if s >= 4 {
                    s = 0;
                    space += 1;
                }
                continue
            }
            else {
                s = 0;
                i += 1;
                cargo_hash.entry(space+i).or_insert(Vec::new()).push(c);
            }
        }
        line
    }
    )
    .collect::<Vec<&str>>();
    

    let moves = input[1]
        .trim()
        .split("\n")
        .collect::<Vec<&str>>();
    let mut moves_numbers: Vec<u32> = vec![];
    for f in moves { 
        let spliter = f.split(" ").collect::<Vec<&str>>();
        moves_numbers.push(spliter[1].parse::<u32>().expect("Valid"));
        moves_numbers.push(spliter[3].parse::<u32>().expect("Valid"));
        moves_numbers.push(spliter[5].parse::<u32>().expect("Valid"));
    }
    let hash_permuted = permut(cargo_hash, moves_numbers);
    
    for f in 0..hash_permuted.len() as u32 {
    println!("{:?}", hash_permuted[&(f+1)][0]);
    }
    1
}

fn main() {
    let file = std::fs::read_to_string("./input")
        .expect("Couldn't read file!!!");
    
    first_part(file);
}

