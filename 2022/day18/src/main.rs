use std::collections::HashMap;
use ordered_float::OrderedFloat;

fn first_part(file: &str) -> i32 {
    
    let mut faces: HashMap<(OrderedFloat<f32>, OrderedFloat<f32>, OrderedFloat<f32>), i32> = HashMap::new();
    let offsets = vec![(0.0, 0.0, 0.5), (0.0, 0.5, 0.0), (0.5, 0.0, 0.0), (0.0, 0.0, -0.5), (0.0, -0.5, 0.0), (-0.5, 0.0, 0.0)];
    
    for line in file.trim().split("\n").collect::<Vec<&str>>() {
        let positions  = line.split(",").map(|l| l.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let (x, y, z) = (positions[0], positions[1], positions[2]);
        
        for (dx, dy, dz) in &offsets {
            let k = (OrderedFloat(x as f32 + dx), OrderedFloat(y as f32 + dy), OrderedFloat(dz + z as f32));
            
            if !(faces.contains_key(&k)){
                faces.insert(k, 0);
            }
            if let Some(value) = faces.get_mut(&k) {
                *value += 1;
            }
        }
    }
    let output: i32 = faces.iter().map(|(_, v)| if *v == 1 { return *v; } else { return 0 as i32; }).sum();
    output
}

fn second_part(file: &str) -> i32 {
    let mut faces: HashMap<(OrderedFloat<f32>, OrderedFloat<f32>, OrderedFloat<f32>), i32> = HashMap::new();
    let offsets = vec![(0.0, 0.0, 0.5), (0.0, 0.5, 0.0), (0.5, 0.0, 0.0), (0.0, 0.0, -0.5), (0.0, -0.5, 0.0), (-0.5, 0.0, 0.0)];
    
    for line in file.trim().split("\n").collect::<Vec<&str>>() {
        let positions  = line.split(",").map(|l| l.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let (x, y, z) = (positions[0], positions[1], positions[2]);
        
        for (dx, dy, dz) in &offsets {
            let k = (OrderedFloat(x as f32 + dx), OrderedFloat(y as f32 + dy), OrderedFloat(dz + z as f32));
            
            if !(faces.contains_key(&k)){
                faces.insert(k, 0);
            }
            if let Some(value) = faces.get_mut(&k) {
                *value += 1;
            }
        }
    }
    let output: i32 = faces.iter().map(|(_, v)| if *v == 1 { return *v; } else { return 0 as i32; }).sum();
    output

}

fn main() {
    let file = std::fs::read_to_string("./input").expect("Couldn't read input file");
    
    println!("Output 1: {}", first_part(&file));
    println!("Output 2: {}", second_part(&file));

}
