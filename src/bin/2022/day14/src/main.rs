use std::collections::HashMap;

#[derive(Default, Debug)]
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

    fn fall(&self, pos: (i32, i32)) -> Option<(i32,i32)> {
        for delta_x in [0, -1, 1] {
            let new_pos = (pos.0 + delta_x, pos.1 + 1);
            if !self.location.contains_key(&new_pos) {
                return Some(new_pos);
            }
        }
        None
    }
}

fn first_part(file: &str) {
    let mut cave = Cave::default();
    
    for line in file.trim().split("\n").collect::<Vec<&str>>() {
        let mut iter = line.split(" -> ");
        let mut start = Cave::convert(iter.next()).unwrap();
        while let Some(end) = Cave::convert(iter.next()) {
            cave.draw_line(start, end);
            start = end;
        }
    }
   
    let mut output = 0;
    while cave.drop_sand() {
        output += 1;
    }

    println!("Output: {}", output);
   
}

fn second_part(file: &str) {
    let mut cave = Cave::default();
    cave.floor = true; 
    for line in file.trim().split("\n").collect::<Vec<&str>>() {
        let mut iter = line.split(" -> ");
        let mut start = Cave::convert(iter.next()).unwrap();
        while let Some(end) = Cave::convert(iter.next()) {
            cave.draw_line(start, end);
            start = end;
        }
    }
   
    let mut output = 0;
    while cave.drop_sand() {
        output += 1;
    }

    println!("Output 2: {}", output);
 
}
fn main() {
    let file = std::fs::read_to_string("./input").expect("Couldn't read input file");
    first_part(&file);  
    second_part(&file); 
}
