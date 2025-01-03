use crate::Runit;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;

pub fn run_2024(day: u32) {
    let mut day01 = day01::AocDay01::new();
    let mut day02 = day02::AocDay02::new();
    let mut day03 = day03::AocDay03::new();
    let mut day04 = day04::AocDay04::new();
    let mut day05 = day05::AocDay05::new();
    let mut day06 = day06::AocDay06::new();
    let mut day07 = day07::AocDay07::new();
    let mut day08 = day08::AocDay08::new();
    let mut day09 = day09::AocDay09::new();
    let mut day10 = day10::AocDay10::new();

    let mut days: Vec<&mut dyn Runit> = vec![
        &mut day01, &mut day02, &mut day03, &mut day04, &mut day05, &mut day06, &mut day07,
        &mut day08, &mut day09, &mut day10,
    ];

    if day == 0 || (day - 1) >= days.len() as u32 {
        println!("Invalid day! it's out of the range");
        std::process::exit(1);
    }

    crate::run(days[day as usize - 1]);
}
