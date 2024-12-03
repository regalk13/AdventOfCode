use crate::Runit;
use regex::Regex;

#[derive(Default)]
pub struct AocDay03 {
    input: String,
}

impl AocDay03 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runit for AocDay03 {
    fn parse(&mut self) {
        let file = crate::read_file("2024".to_string(), "03".to_string());

        self.input = file;
    }

    fn first_part(&mut self) -> String {
        let re = Regex::new(r"mul\((\d+),\s*(\d+)\)").unwrap();

        let result: i32 = re
            .captures_iter(&self.input)
            .map(|s| {
                let first_number = s.get(1).unwrap().as_str();
                let second_number = s.get(2).unwrap().as_str();

                first_number.parse::<i32>().unwrap() * second_number.parse::<i32>().unwrap()
            })
            .sum();
        result.to_string()
    }

    fn second_part(&mut self) -> String {
        let re = Regex::new(r"(don't\(\)|do\(\))|mul\((\d+),\s*(\d+)\)").unwrap();
        let mut do_status = true;
        let results: i32 = re
            .captures_iter(&self.input)
            .map(|s| {
                let option = s.get(0).unwrap().as_str();

                if option == "don't()" || option == "do()" {
                    if let Some(do_fn) = s.get(1) {
                        let do_fn_str = do_fn.as_str();
                        if do_fn_str == "do()" {
                            do_status = true;
                        } else if do_fn_str == "don't()" {
                            do_status = false;
                        }
                    }
                } else {
                    let first_number = s.get(2).unwrap().as_str();
                    let second_number = s.get(3).unwrap().as_str();
                    if do_status {
                        return first_number.parse::<i32>().unwrap()
                            * second_number.parse::<i32>().unwrap();
                    } else {
                        return 0;
                    }
                }
                0
            })
            .sum();
        results.to_string()
    }
}
