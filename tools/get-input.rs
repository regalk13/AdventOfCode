use std::process::Command;
fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    println!("{:?}", args);
    if !(args.len() >= 3) {
        println!("Usage: command [year] [day]");
        std::process::exit(1);
    }
    Command::new("curl")
        .args(&["-b", "session=53616c7465645f5fc2bb71ba269d0119b27573346c155da02aaec5fd94499499fb09e0e727d9508cbf6de563ae583f3301737e3209a3be957587812c5f4084c1", 
              &format!("https://adventofcode.com/{}/day/{}/input", args[1], args[2])])
        .spawn()
        .expect("Failed to get the input file");
}
