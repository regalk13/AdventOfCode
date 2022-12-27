use crate::Runit;

#[derive(Default)]
pub struct AocDay04 {
    input: String,
}
impl AocDay04 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runit for AocDay04 {
    fn parse(&mut self) {
        self.input = crate::read_file("2022".to_string(), "04".to_string());
    }
    fn second_part(&mut self) -> String {
        self.input
            .lines()
            .map(|line| {
                let (first_pair, second_pair) = line.split_once(",").unwrap();

                let first_pair = first_pair.split("-").collect::<Vec<&str>>();
                let second_pair = second_pair.split("-").collect::<Vec<&str>>();
                let a = first_pair[0].parse::<u32>().expect("Valid");
                let b = first_pair[1].parse::<u32>().expect("Valid");
                let c = second_pair[0].parse::<u32>().expect("Valid");
                let d = second_pair[1].parse::<u32>().expect("Valid");

                if (b < c || a > d) == false {
                    return 1;
                } else {
                    return 0;
                }
            })
            .sum::<u32>()
            .to_string()
    }

    fn first_part(&mut self) -> String {
        self.input
            .lines()
            .map(|line| {
                let (first_pair, second_pair) = line.split_once(",").unwrap();

                let first_pair = first_pair.split("-").collect::<Vec<&str>>();
                let second_pair = second_pair.split("-").collect::<Vec<&str>>();
                let first_pairs = first_pair[0].parse::<u32>().expect("Valid");
                let first_pairs2 = first_pair[1].parse::<u32>().expect("Valid");
                let second_pairs = second_pair[0].parse::<u32>().expect("Valid");
                let second_pairs2 = second_pair[1].parse::<u32>().expect("Valid");

                if first_pairs >= second_pairs && first_pairs2 <= second_pairs2 {
                    return 1;
                }

                if second_pairs >= first_pairs && second_pairs2 <= first_pairs2 {
                    return 1;
                } else {
                    return 0;
                }
            })
            .sum::<u32>()
            .to_string()
    }
}
