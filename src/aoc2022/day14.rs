use crate::Runit;

use std::collections::HashMap;

#[derive(Default)]
pub struct AocDay14 {
    cave: Cave,
}

impl AocDay14 {
    pub fn new() -> Self {
        AocDay14::default()
    }
}

#[derive(Default, Debug, Clone)]
struct Cave {
    location: HashMap<(i32, i32), char>,
    bottom: i32,
    floor: bool,
}

impl Cave {
    fn convert(s: Option<&str>) -> Option<(i32, i32)> {
        if let Some(s) = s {
            let (x, y) = s.split_once(",").unwrap();
            Some((x.parse().unwrap(), y.parse().unwrap()))
        } else {
            None
        }
    }

    fn draw_line(&mut self, start: (i32, i32), end: (i32, i32)) {
        let dx = (end.0 - start.0).signum();
        let dy = (end.1 - start.1).signum();

        self.bottom = self.bottom.max(start.1.max(end.1));

        let mut point = start;
        self.location.insert(point, '#');
        while point != end {
            point.0 += dx;
            point.1 += dy;
            self.location.insert(point, '#');
        }
    }

    fn drop_sand(&mut self) -> bool {
        let mut sand: (i32, i32) = (500, 0);

        if self.floor && self.location.contains_key(&sand) {
            return false;
        }

        while let Some(next_pos) = self.fall(sand) {
            if !self.floor && next_pos.1 > self.bottom {
                return false;
            }
            sand = next_pos;
            if self.floor && sand.1 == self.bottom + 1 {
                break;
            }
        }

        self.location.insert(sand, 'o');
        true
    }

    fn fall(&self, pos: (i32, i32)) -> Option<(i32, i32)> {
        for delta_x in [0, -1, 1] {
            let new_pos = (pos.0 + delta_x, pos.1 + 1);
            if !self.location.contains_key(&new_pos) {
                return Some(new_pos);
            }
        }
        None
    }
}

impl Runit for AocDay14 {
    fn parse(&mut self) {
        let mut cave = Cave::default();
        let file = crate::read_file("2022".to_string(), "14".to_string());
        for line in file.trim().split("\n").collect::<Vec<&str>>() {
            let mut iter = line.split(" -> ");
            let mut start = Cave::convert(iter.next()).unwrap();
            while let Some(end) = Cave::convert(iter.next()) {
                cave.draw_line(start, end);
                start = end;
            }
        }
        self.cave = cave;
    }

    fn first_part(&mut self) -> String {
        let mut output = 0;
        let mut cave = self.cave.clone();
        while cave.drop_sand() {
            output += 1;
        }

        output.to_string()
    }

    fn second_part(&mut self) -> String {
        self.cave.floor = true;
        let mut cave = self.cave.clone();
        let mut output = 0;

        while cave.drop_sand() {
            output += 1;
        }

        output.to_string()
    }
}
