use crate::Runit;
use std::collections::{HashMap, HashSet};
#[derive(Default)]
pub struct AocDay05 {
    lines: Vec<String>,
}

impl AocDay05 {
    pub fn new() -> Self {
        Self::default()
    }
}

fn sort_update(line: &mut Vec<String>, rules: &HashMap<i32, HashSet<i32>>) {
    line.sort_by(|a, b| {
        let a = a.parse::<i32>().unwrap();
        let b = b.parse::<i32>().unwrap();

        if rules.get(&a).map_or(false, |set| set.contains(&b)) {
            std::cmp::Ordering::Less
        } else if rules.get(&b).map_or(false, |set| set.contains(&a)) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });
}

impl Runit for AocDay05 {
    fn parse(&mut self) {
        let file = crate::read_file("2024".to_string(), "05".to_string());

        self.lines = file
            .split_whitespace()
            .filter(|l| !l.is_empty())
            .map(|line| line.to_string())
            .collect::<Vec<String>>();
    }

    fn first_part(&mut self) -> String {
        let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
  
        self.lines
        .iter()
        .filter(|l| l.contains("|"))
        .for_each(|line| {
            if let Some((x, y)) = line.split_once('|') {
                let x: i32 = x.parse().unwrap();
                let y: i32 = y.parse().unwrap();
                rules.entry(x).or_insert_with(Vec::new).push(y);
            }
        });

        let pages = self.lines
        .iter()
        .filter(|l| !l.contains("|"))
        .map(|l| l.split(",").map(|s| s.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();

        let result = pages
        .into_iter()
        .filter(|line| {
            let mut result = true;
            for (i, n) in line.into_iter().enumerate() {
                if !(i < line.len() -1) {
                    break
                }
                let matches = rules.entry(n.parse::<i32>().unwrap()).or_insert_with(Vec::new);
                let next = line[i+1].parse::<i32>().unwrap();
                if !matches.contains(&next) {
                    result = false;
                    break;
                }
            }

            return result;
        })
        .map(|line| {
            return line[line.len()/2].parse::<i32>().unwrap()
        })
        .sum::<i32>();

        result.to_string()
    }

    fn second_part(&mut self) -> String {
        let mut rules: HashMap<i32, HashSet<i32>> = HashMap::new();
  
        self.lines
        .iter()
        .filter(|l| l.contains("|"))
        .for_each(|line| {
            if let Some((x, y)) = line.split_once('|') {
                let x: i32 = x.parse().unwrap();
                let y: i32 = y.parse().unwrap();
                rules.entry(x).or_insert_with(HashSet::new).insert(y);            
            }
        });

        let pages = self.lines
        .iter()
        .filter(|l| !l.contains("|"))
        .map(|l| l.split(",").map(|s| s.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();

        let result = pages
        .into_iter()
        .filter(|line| {
            let mut result = false;
            for (i, n) in line.into_iter().enumerate() {
                if !(i < line.len() -1) {
                    break
                }
                let matches = rules.entry(n.parse::<i32>().unwrap()).or_insert_with(HashSet::new);
                let next = line[i+1].parse::<i32>().unwrap();
                if !matches.contains(&next) {
                    result = true;
                    break;
                }
            }
            
            return result;
        })
        .collect::<Vec<Vec<String>>>();


        result
        .iter()
        .map(|r| {
            let mut sorted = r.clone();
            sort_update(&mut sorted, &rules);
            sorted[sorted.len()/2].parse::<i32>().unwrap()
        }).sum::<i32>().to_string()

    }
}
