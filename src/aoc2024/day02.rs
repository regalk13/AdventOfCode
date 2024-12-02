use crate::Runit;

#[derive(Default)]
pub struct AocDay02 {
    input: Vec<Vec<i32>>,
}

impl AocDay02 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runit for AocDay02 {
    fn parse(&mut self) {
        let file = crate::read_file("2024".to_string(), "02".to_string());

        self.input = file
            .split("\n")
            .filter(|s| !s.is_empty())
            .map(|s| {
                s.split_whitespace()
                    .map(|i| i.parse::<i32>().unwrap())
                    .collect()
            })
            .collect();
    }

    fn first_part(&mut self) -> String {
        fn is_safe(f: &[i32]) -> bool {
            let increasing = f[0] < f[f.len() - 1];

            for n in 1..f.len() {
                let diff = f[n] - f[n - 1];
                if increasing {
                    if !(1 <= diff && diff <= 3) {
                        return false;
                    }
                } else {
                    if !(-3 <= diff && diff <= -1) {
                        return false;
                    }
                }
            }
            true
        }

        let response = self
            .input
            .iter()
            .filter(|f| {
                if is_safe(f) {
                    return true;
                }
                false
            })
            .count();

        response.to_string()
    }

    fn second_part(&mut self) -> String {
        fn is_safe(f: &[i32]) -> bool {
            let increasing = f[0] < f[f.len() - 1];

            for n in 1..f.len() {
                let diff = f[n] - f[n - 1];
                if increasing {
                    if !(1 <= diff && diff <= 3) {
                        return false;
                    }
                } else {
                    if !(-3 <= diff && diff <= -1) {
                        return false;
                    }
                }
            }
            true
        }

        let response = self
            .input
            .iter()
            .filter(|f| {
                if is_safe(f) {
                    return true;
                }

                for i in 0..f.len() {
                    let mut temp = f.to_vec();
                    temp.remove(i);
                    if is_safe(&temp) {
                        return true;
                    }
                }
                false
            })
            .count();

        response.to_string()
    }
}
