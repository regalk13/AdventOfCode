use crate::Runit;

#[derive(Default)]
pub struct AocDay01 {
    output: Vec<u32>,
}
impl AocDay01 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runit for AocDay01 {
    fn parse(&mut self) {
        let file = advent_of_code::read_file("2022".to_string(), "01".to_string());
        self.output = file
            .trim()
            .split("\n\n")
            .map(|line| {
                line.split("\n")
                    .map(|l| l.parse::<u32>().unwrap())
                    .sum::<u32>()
            })
            .collect::<Vec<u32>>();
    }

    fn first_part(&mut self) -> String {
        self.output.iter().max().unwrap().to_string()
    }

    fn second_part(&mut self) -> String {
        self.output.sort_by(|a, b| b.cmp(a));
        self.output.iter().take(3).sum::<u32>().to_string()
    }
}
