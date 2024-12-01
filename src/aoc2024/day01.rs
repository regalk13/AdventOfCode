use crate::Runit;

#[derive(Default)]
pub struct AocDay01 {
    left: Vec<i32>,
    right: Vec<i32>,
}

impl AocDay01 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runit for AocDay01 {
    fn parse(&mut self) {
        let file = crate::read_file("2024".to_string(), "01".to_string());

        let lines: Vec<String> = file
            .split("\n")
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();

        lines.into_iter().for_each(|s| {
            let values: Vec<i32> = s
                .split_whitespace()
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<i32>().unwrap())
                //.map(|s| s.to_string())
                .collect();
            self.left.push(values[0]);
            self.right.push(values[1])
        });
    }

    fn first_part(&mut self) -> String {
        self.left.sort_unstable();
        self.right.sort_unstable();

        let response: i32 = self
            .left
            .iter()
            .zip(&self.right)
            .map(|(left, right)| (right - left).abs())
            .sum();

        response.to_string()
    }

    fn second_part(&mut self) -> String {
        let response: i32 = self
            .left
            .iter()
            .map(|n| n * self.right.iter().filter(|l| *l == n).count() as i32)
            .sum();

        response.to_string()
    }
}
