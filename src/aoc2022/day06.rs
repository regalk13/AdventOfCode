use crate::Runit;
use std::collections::HashSet;

#[derive(Default)]
pub struct AocDay06 {
    chars: Vec<char>,
}
impl AocDay06 {
    pub fn new() -> Self {
        Self::default()
    }
}
fn check_duplicates(s: Vec<char>) -> bool {
    let mut seen = HashSet::new();

    for c in s {
        let contained = seen.contains(&c);
        seen.insert(c);
        if contained {
            return false;
        }
    }
    true
}

impl Runit for AocDay06 {
    fn parse(&mut self) {
        let file = crate::read_file("2022".to_string(), "06".to_string());
        self.chars = file.chars().collect::<Vec<char>>();
    }

    fn first_part(&mut self) -> String {
        for (i, _) in self.chars.iter().enumerate() {
            if i <= self.chars.len() - 4 {
                if check_duplicates(self.chars[i..=i + 3].to_vec()) {
                    return (i + 4).to_string();
                }
            }
        }
        return "".to_string();
    }
    fn second_part(&mut self) -> String {
        for (i, _) in self.chars.iter().enumerate() {
            if i <= self.chars.len() - 14 {
                if check_duplicates(self.chars[i..=i + 13].to_vec()) {
                    return (i + 14).to_string();
                }
            }
        }
        return "".to_string();
    }
}
