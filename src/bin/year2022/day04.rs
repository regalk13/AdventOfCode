
fn second_part(file: String) -> u32 {
    file
    .lines()
    .map(|line| {
        let lines: Vec<&str> = line.split(",").collect();

        let first_pair = lines[0].split("-").collect::<Vec<&str>>();
        let second_pair = lines[1].split("-").collect::<Vec<&str>>();
        let a = first_pair[0].parse::<u32>().expect("Valid");  
        let b = first_pair[1].parse::<u32>().expect("Valid");  
        let c = second_pair[0].parse::<u32>().expect("Valid");  
        let d = second_pair[1].parse::<u32>().expect("Valid"); 
        
        if (b < c || a > d) == false {
            println!("Lines: {}:{}, {},{}", a,b,c,d);
            return 1;
        } 

        else {
            return 0;
        }
        
    }).sum::<u32>()    
}

fn first_part(file: String) -> u32 {
    file
    .lines()
    .map(|line| {
        let lines: Vec<&str> = line.split(",").collect();

        let first_pair = lines[0].split("-").collect::<Vec<&str>>();
        let second_pair = lines[1].split("-").collect::<Vec<&str>>();
        let first_pairs = first_pair[0].parse::<u32>().expect("Valid");  
        let first_pairs2 = first_pair[1].parse::<u32>().expect("Valid");  
        let second_pairs = second_pair[0].parse::<u32>().expect("Valid");  
        let second_pairs2 = second_pair[1].parse::<u32>().expect("Valid"); 

        if first_pairs >= second_pairs && first_pairs2 <= second_pairs2 {
            return 1;
        }
        
        if second_pairs >= first_pairs && second_pairs2  <= first_pairs2 {
            return 1;
        }

        else {
            return 0;
        }
        
    }).sum::<u32>()    
}

fn main() {
    let file = std::fs::read_to_string("./input")
        .expect("Couldn't read input");

    println!("Result: {}", first_part(file));

}
