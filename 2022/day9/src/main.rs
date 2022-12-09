use std::str::FromStr;
use std::collections::HashSet;

#[derive(Debug, Clone)]
enum Moves {
    Up = 1,
    Down = 2,
    Left = 3,
    Right = 4
}

impl FromStr for Moves {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Moves::Up),
            "D" => Ok(Moves::Down),
            "L" => Ok(Moves::Left),
            "R" => Ok(Moves::Right),
            _ => Err("Invalid Move".to_string())

        }

    }
}

fn touching(x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
    return (x1 - x2).abs() <= 1 && (y1 - y2).abs() <= 1;
}

fn move_it(hx: i32, hy: i32, tx: i32, ty: i32) -> (i32, i32) {
    let mut sign_x = 0;
    let mut sign_y = 0;
    let mut temp_tx = 0;
    let mut temp_ty = 0;
    if !(touching(hx, hy, tx, ty)) {
        if hx == tx {
            sign_x = 0;
        }
        else {
            sign_x = (hx - tx) / (hx - tx).abs();
        }
        
        if hy == ty {
            sign_y = 0;
        }
        else {
            sign_y = (hy - ty) / (hy - ty).abs();
        }
        temp_tx += sign_x; 
        temp_ty += sign_y;
    }
    (temp_tx, temp_ty)
}

fn first_part(file: String) -> u32 {
    let moves = file.trim().split("\n").collect::<Vec<&str>>();
    
    let mut hx = 0;
    let mut hy = 0;
    let mut tx = 0;
    let mut ty = 0;

    let mut tail_visited = HashSet::new();
    tail_visited.insert((tx, ty));

    for line in moves {
        let (op, amount) = line.split_once(" ").unwrap();
        
        let range_amount = amount.parse::<i32>().unwrap();
        let ds = op.parse::<Moves>().unwrap();
        let ds_moves = match ds {
            Moves::Up => [0, 1],
            Moves::Down => [0, -1],
            Moves::Left => [-1, 0],
            Moves::Right => [1, 0]
        };

        for _  in 0..range_amount {
            hx += ds_moves[0];
            hy += ds_moves[1];
            let moves_made = move_it(hx, hy, tx, ty);
            tx += moves_made.0;
            ty += moves_made.1;
            tail_visited.insert((tx, ty));
        }
    }
    tail_visited.len() as u32 
}

fn main() {
    let file = std::fs::read_to_string("./input").expect("Couldn't read input file");
   
   println!("Result: {:?}", first_part(file));
}
