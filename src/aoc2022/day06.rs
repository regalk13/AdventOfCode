use std::collections::HashSet;

fn check_duplicates(s: Vec<char>) -> bool {
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
    let file = std::fs::read_to_string("./input").expect("Couldn't read the file!");

    let result = file.trim().lines().collect::<Vec<&str>>();
    let chars = result[0].chars().collect::<Vec<char>>();

    for (i, _f) in chars.iter().enumerate() {
        // Part 1
        // if i <= chars.len() - 4
        // Part 2
        if i <= chars.len() - 14 {
            // Part 1
            // chec_duplicates(chars[i..=i+3].to_vec()
            if check_duplicates(chars[i..=i + 13].to_vec()) {
                // Part 1 &chars[i..=i+4
                // println!("Find: {:?} : {:?}", &chars[i..=i+13], check_duplicates(chars[i..=i+13].to_vec()));
                // Part 1 i+4
                println!("Index: {}", i + 14);
                return;
            }
        }
    }
}
