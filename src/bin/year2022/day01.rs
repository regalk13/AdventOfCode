pub fn main() {
    let file = advent_of_code::read_file("2022".to_string(), "01".to_string());

    let mut i = 0;
    let mut vector: Vec<u32> = Vec::new();

    for l in file.split("\n") {
        if l.is_empty() {
            vector.push(0);
            i += 1;
            continue;
        }
        let l: u32 = l.trim().parse().expect("Expected a number");
        if vector.len() <= 0 {
            vector.push(l);
        }
        vector[i] = vector[i] + l;
    }

    vector.sort_by(|a, b| b.cmp(a));

    println!("{}", vector[0]);
    // Part 2
    println!("The sum of top three is: {}", vector.iter().take(3).sum::<u32>());
}
