use crate::Runit;
mod day01;
pub fn run_2024(day: u32) {
    let mut day01 = day01::AocDay01::new();

    let mut days: Vec<&mut dyn Runit> = vec![&mut day01];

    if day == 0 || (day - 1) >= days.len() as u32 {
        println!("Invalid day! it's out of the range");
        std::process::exit(1);
    }

    crate::run(days[day as usize - 1]);
}
