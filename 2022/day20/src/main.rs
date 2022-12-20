// Parsing the input, this returns a vector of tuples (0 <- index, 1 <- value)
fn parse(input: &str) -> Vec<(i32, i32)> {
    input
        .trim()
        .split("\n")
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .iter()
        .enumerate()
        .map(|(i, j)| (i as i32, *j))
        .collect::<Vec<(i32, i32)>>()
}

fn first_part(file: &str) -> i32 {
    let parsed = parse(file);
    // Clone of the parsed input, mutable for changes
    let mut output = parsed.clone();
    
    for (i, _) in parsed.iter() {
        // Position of the current index in the loop
        let position = output.iter().position(|&x| x.0 == *i as i32).unwrap();
        // Get and remove the value
        let current = output.remove(position);
        // Index + value
        let add = position as i64 + current.1 as i64;
        
        // Get the value euclid include negative positions! 
        let new_pos = add.rem_euclid(output.len() as i64);
        // println!("Current position: {:?} pos: {}", current, new_pos);
        output.insert(new_pos as usize, current);
    }
    // Position of zero
    let zero = output.iter().position(|v| v.1 == 0).unwrap();
    // 1000, 2000 and 3000 cycles from 0 
    let a = output[(1000 + zero) % output.len()].1;
    let b = output[(2000 + zero) % output.len()].1;
    let c = output[(3000 + zero) % output.len()].1;
    // return the sum!
    a + b + c as i32
}

fn second_part(file: &str) -> i64 {
    // Decrypt key
    let d_key = 811589153;
    // Multiply each time by d_key and convert it to i64
    let parsed = parse(file)
        .iter()
        .map(|(i, s)| (*i as i64, *s as i64 * d_key as i64))
        .collect::<Vec<(i64, i64)>>();
    let mut output = parsed.clone();
   // move items 10 times
    for _ in 0..10 {
        for (i, _) in parsed.iter() {
            let position = output.iter().position(|&x| x.0 == *i as i64).unwrap();

            let current = output.remove(position);
            let add = position as i64 + current.1 as i64;

            let new_pos = add.rem_euclid(output.len() as i64);
            // println!("Current position: {:?} pos: {}", current, new_pos);
            output.insert(new_pos as usize, current);
        }
    }
    // zero position
    let zero = output.iter().position(|v| v.1 == 0).unwrap();
    // 1000, 2000 and 3000 cycles from 0 
    let a = output[(1000 + zero) % output.len()].1;
    let b = output[(2000 + zero) % output.len()].1;
    let c = output[(3000 + zero) % output.len()].1;

    a + b + c as i64
}

fn main() {
    let file = std::fs::read_to_string("./input").expect("Couldn't read input file");

    println!("Output 1: {}", first_part(&file));
    println!("Output 2: {}", second_part(&file));
}
