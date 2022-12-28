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
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

pub fn run_2022(day: u32) {
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
    let mut day11 = day11::AocDay11::new();
    let mut day12 = day12::AocDay12::new();
    let mut day13 = day13::AocDay13::new();
    let mut day14 = day14::AocDay14::new();
    let mut day15 = day15::AocDay15::new();
    let mut day16 = day16::AocDay16::new();
    let mut day17 = day17::AocDay17::new();
    let mut day18 = day18::AocDay18::new();
    let mut day19 = day19::AocDay19::new();
    let mut day20 = day20::AocDay20::new();
    let mut day21 = day21::AocDay21::new();
    let mut day22 = day22::AocDay22::new();
    let mut day23 = day23::AocDay23::new();
    let mut day24 = day24::AocDay24::new();
    let mut day25 = day25::AocDay25::new();

    let mut days: Vec<&mut dyn Runit> = vec![
        &mut day01, &mut day02, &mut day03, &mut day04, &mut day05, &mut day06, &mut day07,
        &mut day08, &mut day09, &mut day10, &mut day11, &mut day12, &mut day13, &mut day14,
        &mut day15, &mut day16, &mut day17, &mut day18, &mut day19, &mut day20, &mut day21,
        &mut day22, &mut day23, &mut day24, &mut day25,
    ];

    if day == 0 || (day - 1) >= days.len() as u32 {
        println!("Invalid day! it's out of the range");
        std::process::exit(1);
    }

    crate::run(days[day as usize - 1]);
}
