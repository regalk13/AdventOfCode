use crate::Runit;
use aho_corasick::AhoCorasick;

#[derive(Default)]
pub struct AocDay01 {
    output: Vec<String>,
}

impl AocDay01 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runit for AocDay01 {
    fn parse(&mut self) {
        let file = crate::read_file("2023".to_string(), "01".to_string());

        self.output = file
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect()
    }

    fn first_part(&mut self) -> String {
        let mut total: i32 = 0;

        for line in &self.output {
            let numbers = line
                .chars()
                .filter(|ch| ch.is_ascii_digit())
                .map(|ch| ch as u8 - b'0')
                .collect::<Vec<_>>();
            let first = numbers.iter().nth(0).unwrap();
            let last = numbers.iter().last().unwrap();
            total += (first.to_string() + &last.to_string())
                .parse::<i32>()
                .unwrap();
        }

        return total.to_string();
    }

    fn second_part(&mut self) -> String {
        let nums = [
            "one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6", "seven",
            "7", "eight", "8", "nine", "9",
        ];
        let mut total = 0;

        let ac = AhoCorasick::new(nums).unwrap();

        for line in &self.output {
            let matches = ac.find_overlapping_iter(line).collect::<Vec<_>>();
            let first = matches.iter().nth(0).unwrap().pattern().as_usize() / 2 + 1;
            let last = matches.iter().last().unwrap().pattern().as_usize() / 2 + 1;

            total += 10 * first as i32 + last as i32;
        }
        return total.to_string();
    }
}
