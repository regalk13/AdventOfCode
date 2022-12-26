use std::process::Command;
fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    println!("{:?}", args);
    if !(args.len() >= 3) {
        println!("Usage: command [year] [day]");
        std::process::exit(1);
    }
    Command::new("curl")
        .args(&["-b", "session=COOKIE", 
              &format!("https://adventofcode.com/{}/day/{}/input", args[1], args[2])])
        .spawn()
        .expect("Failed to get the input file");
}
