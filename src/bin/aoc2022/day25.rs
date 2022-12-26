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

fn first_part(file: &str) {
    let number_snafu = file.trim().lines().map(|i| snafu_decimal(i)).sum::<i64>();
    let decimal = decimal_snafu(number_snafu);
    println!("Output: {}", decimal);
    println!("Merry christmas!");
}

fn main() {
    let file = std::fs::read_to_string("./input").expect("Couldn't read input file");

    first_part(&file);
}
