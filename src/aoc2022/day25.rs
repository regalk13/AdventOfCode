use crate::Runit;

#[derive(Default)]
pub struct AocDay25 {
    file: String,
}

impl AocDay25 {
    pub fn new() -> Self {
        AocDay25::default()
    }
}

fn snafu_decimal(number: &str) -> i64 {
    let mut place: i64 = 1;
    let mut result: i64 = 0;
    for n in number.chars().rev() {
        if n.is_digit(10) {
            result += n.to_digit(10).unwrap() as i64 * place;
        } else {
            match n {
                '-' => result += -1 * place,
                '=' => result += -2 * place,
                _ => (),
            }
        }
        place *= 5;
    }

    result
}

fn decimal_snafu(mut number: i64) -> String {
    let mut total = "".to_string();
    while number > 0 {
        let rem = number % 5;
        number /= 5;
        if rem <= 2 {
            total += &rem.to_string();
        } else {
            let remer = "   =-".chars().collect::<Vec<char>>();
            total += &remer[rem as usize].to_string();
            number += 1;
        }
    }
    total.chars().rev().collect::<String>()
}

impl Runit for AocDay25 {
    fn parse(&mut self) {
        self.file = crate::read_file("2022".to_string(), "25".to_string());
    }
    fn first_part(&mut self) -> String {
        let number_snafu = self
            .file
            .trim()
            .lines()
            .map(|i| snafu_decimal(i))
            .sum::<i64>();
        let decimal = decimal_snafu(number_snafu);
        decimal.to_string()
    }
    fn second_part(&mut self) -> String {
        "You make a smoothie with all fifty stars and deliver it to the reindeer! The sleigh is already warmed up by the time they finish eating.".to_string()
    }
}
