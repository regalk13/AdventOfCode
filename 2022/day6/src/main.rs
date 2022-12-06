use std::collections::HashSet;

fn check_duplicates(mut s: Vec<char>) -> bool {
    let mut seen = HashSet::new();
    
    for c in s {
        let contained = seen.contains(&c);
        seen.insert(c);
        if contained {
            return false;
        }
    }
    true
}
fn main() {
    let file = std::fs::read_to_string("./input")
        .expect("Couldn't read the file!");
    
    let result = file
        .trim()
        .lines()
        .collect::<Vec<&str>>();
    let chars = result[0].chars().collect::<Vec<char>>();

    for (i, f) in chars.iter().enumerate() {
        if i <= chars.len() - 14 {
            if check_duplicates(chars[i..=i+13].to_vec()) {
                println!("Find: {:?} : {:?}", &chars[i..=i+13], check_duplicates(chars[i..=i+13].to_vec()));
                println!("Index: {}", i+14);
                return;

            }
        }
    }
    println!("Result: {:?}", result);
}
