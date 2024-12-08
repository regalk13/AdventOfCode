use crate::Runit;

#[derive(Default)]
pub struct AocDay07 {
    lines: Vec<Calibration>,
}

#[derive(Clone)]
struct Calibration {
    target: i64,
    values: Vec<i64>,
}

impl AocDay07 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl AocDay07 {
    fn obtain(&mut self, target: i64, values: &[i64], second_p: bool) -> bool {
        let n = values.len();

        if n == 1 {
            return target == values[0];
        }

        if target % values[n - 1] == 0
            && self.obtain(target / values[n - 1], &values[..n - 1], second_p)
        {
            return true;
        }

        if target > values[n - 1] && self.obtain(target - values[n - 1], &values[..n - 1], second_p)
        {
            return true;
        }

        if second_p {
            let s_target = target.to_string();
            let s_last = values[n - 1].to_string();

            if s_target.len() > s_last.len() && s_target.ends_with(&s_last) {
                let truncated = &s_target[..s_target.len() - s_last.len()];
                if let Ok(truncated_value) = truncated.parse::<i64>() {
                    if self.obtain(truncated_value, &values[..n - 1], second_p) {
                        return true;
                    }
                }
            }
        }

        false
    }
}

impl Runit for AocDay07 {
    fn parse(&mut self) {
        let file = crate::read_file("2024".to_string(), "07".to_string());

        self.lines = file
            .split("\n")
            .filter(|s| !s.is_empty())
            .map(|s| {
                let line: Vec<_> = s.split(": ").collect();
                let target = line[0].parse::<i64>().unwrap();
                let values = line[1]
                    .split(" ")
                    .filter(|s| !s.is_empty())
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect();
                Calibration { target, values }
            })
            .collect();
    }

    fn first_part(&mut self) -> String {
        self.lines
            .clone()
            .iter()
            .filter(|calibration| self.obtain(calibration.target, &calibration.values, false))
            .map(|calibration| calibration.target)
            .sum::<i64>()
            .to_string()
    }

    fn second_part(&mut self) -> String {
        self.lines
            .clone()
            .into_iter()
            .filter(|calibration| self.obtain(calibration.target, &calibration.values, true))
            .map(|calibration| calibration.target)
            .sum::<i64>()
            .to_string()
    }
}
