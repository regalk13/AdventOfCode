use std::collections::HashSet;

fn convert_number(s: char) -> String {
    s.to_lowercase()
    .filter( |&letter| { letter as u8 >= 97 && letter as u8 <= 122 } )
    .map( |letter| { (letter as u8 - 96).to_string() } )
    .collect::<Vec<String>>()
    .join("")
}

fn main() {
    let file = std::fs::read_to_string("./input")
    .unwrap();

    let result = file
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
        .sum::<u32>();
    
    println!("Result: {}", result);
}
