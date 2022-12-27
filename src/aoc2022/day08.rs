use crate::Runit;

#[derive(Default)]
pub struct AocDay08 {
    input: Vec<String>,
}
impl AocDay08 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runit for AocDay08 {
    fn parse(&mut self) {
        let file = crate::read_file("2022".to_string(), "08".to_string());
        self.input = file.split("\n").map(|s| s.to_string()).collect::<Vec<String>>();
    }
    fn second_part(&mut self) -> String {
        let binding = &self.input;
        let mut visibles = vec![];
        for (i, f) in binding.iter().enumerate() {
            for (n, j) in f.chars().collect::<Vec<char>>().iter().enumerate() {
                if n <= 0 || n >= binding.len() - 2 || i <= 0 || i >= binding.len() - 2 {
                    continue;
                }
                let current = j.to_digit(10).unwrap();
                let up = binding[i - 1].chars().collect::<Vec<char>>()[n]
                    .to_digit(10)
                    .unwrap();
                let down = binding[i + 1].chars().collect::<Vec<char>>()[n]
                    .to_digit(10)
                    .unwrap();
                let left = binding[i].chars().collect::<Vec<char>>()[n - 1]
                    .to_digit(10)
                    .unwrap();
                let right = binding[i].chars().collect::<Vec<char>>()[n + 1]
                    .to_digit(10)
                    .unwrap();

                let mut count_up = 0;
                let mut count_down = 0;
                let mut count_right = 0;
                let mut count_left = 0;

                if up < current {
                    let mut m = 1;
                    while i >= m {
                        if binding[i - m].chars().collect::<Vec<char>>()[n]
                            .to_digit(10)
                            .unwrap()
                            < current
                        {
                            count_up += 1;
                            m += 1;
                        } else {
                            count_up += 1;
                            break;
                        }
                    }
                } else {
                    count_up += 1
                }

                if down < current {
                    let mut m = 1;
                    while m <= (binding.len() - 2 - i) {
                        if binding[i + m].chars().collect::<Vec<char>>()[n]
                            .to_digit(10)
                            .unwrap()
                            < current
                        {
                            count_down += 1;
                            m += 1;
                        } else {
                            count_down += 1;
                            break;
                        }
                    }
                } else {
                    count_down += 1
                }

                if right < current {
                    let mut m = 1;
                    while (n + m) <= (binding.len() - 2) {
                        if binding[i].chars().collect::<Vec<char>>()[n + m]
                            .to_digit(10)
                            .unwrap()
                            < current
                        {
                            count_right += 1;
                            m += 1;
                        } else {
                            count_right += 1;
                            break;
                        }
                    }
                } else {
                    count_right += 1;
                }

                if left < current {
                    let mut m = 1;
                    while n >= m {
                        if binding[i].chars().collect::<Vec<char>>()[n - m]
                            .to_digit(10)
                            .unwrap()
                            < current
                        {
                            count_left += 1;
                            m += 1;
                        } else {
                            count_left += 1;
                            break;
                        }
                    }
                } else {
                    count_left += 1;
                }
                visibles.push(count_up * count_down * count_right * count_left);
            }
        }
        visibles.iter().max().unwrap().to_string()
    }

    fn first_part(&mut self) -> String {
        let binding = &self.input;
        let mut visibles = 0;
        for (i, f) in binding.iter().enumerate() {
            for (n, j) in f.chars().collect::<Vec<char>>().iter().enumerate() {
                if n <= 0 || n >= binding.len() - 2 || i <= 0 || i >= binding.len() - 2 {
                    visibles += 1;
                    continue;
                }
                let current = j.to_digit(10).unwrap();
                let up = binding[i - 1].chars().collect::<Vec<char>>()[n]
                    .to_digit(10)
                    .unwrap();
                let down = binding[i + 1].chars().collect::<Vec<char>>()[n]
                    .to_digit(10)
                    .unwrap();
                let left = binding[i].chars().collect::<Vec<char>>()[n - 1]
                    .to_digit(10)
                    .unwrap();
                let right = binding[i].chars().collect::<Vec<char>>()[n + 1]
                    .to_digit(10)
                    .unwrap();

                let mut visible_up = false;
                let mut visible_down = false;
                let mut visible_left = false;
                let mut visible_right = false;

                if up < current {
                    let mut m = 1;
                    while i >= m {
                        if binding[i - m].chars().collect::<Vec<char>>()[n]
                            .to_digit(10)
                            .unwrap()
                            < current
                        {
                            visible_up = true;
                            m += 1;
                        } else {
                            visible_up = false;
                            break;
                        }
                    }
                    if visible_up {
                        visibles += 1;
                        continue;
                    }
                }

                if down < current {
                    let mut m = 1;
                    while m <= (binding.len() - 2 - i) {
                        if binding[i + m].chars().collect::<Vec<char>>()[n]
                            .to_digit(10)
                            .unwrap()
                            < current
                        {
                            visible_down = true;
                            m += 1;
                        } else {
                            visible_down = false;
                            break;
                        }
                    }
                    if visible_down {
                        visibles += 1;
                        continue;
                    }
                }

                if right < current {
                    let mut m = 1;
                    while (n + m) <= (binding.len() - 2) {
                        if binding[i].chars().collect::<Vec<char>>()[n + m]
                            .to_digit(10)
                            .unwrap()
                            < current
                        {
                            visible_right = true;
                            m += 1;
                        } else {
                            visible_right = false;
                            break;
                        }
                    }
                    if visible_right {
                        visibles += 1;
                        continue;
                    }
                }

                if left < current {
                    let mut m = 1;
                    while n >= m {
                        if binding[i].chars().collect::<Vec<char>>()[n - m]
                            .to_digit(10)
                            .unwrap()
                            < current
                        {
                            visible_left = true;
                            m += 1;
                        } else {
                            visible_left = false;
                            break;
                        }
                    }
                    if visible_left {
                        visibles += 1;
                        continue;
                    }
                }
            }
        }
        visibles.to_string()
    }
}
