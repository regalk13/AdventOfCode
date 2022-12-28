use crate::Runit;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Default)]
pub struct AocDay09 {
    moves: Vec<String>,
}

impl AocDay09 {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone)]
enum Moves {
    Up = 1,
    Down = 2,
    Left = 3,
    Right = 4,
}

impl FromStr for Moves {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Moves::Up),
            "D" => Ok(Moves::Down),
            "L" => Ok(Moves::Left),
            "R" => Ok(Moves::Right),
            _ => Err("Invalid Move".to_string()),
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
        } else {
            sign_x = (hx - tx) / (hx - tx).abs();
        }

        if hy == ty {
            sign_y = 0;
        } else {
            sign_y = (hy - ty) / (hy - ty).abs();
        }
        temp_tx += sign_x;
        temp_ty += sign_y;
    }
    (temp_tx, temp_ty)
}

fn move_second(dx: i32, dy: i32, mut knots: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut sign_x = 0;
    let mut sign_y = 0;
    knots[0].0 += dx;
    knots[0].1 += dy;

    for i in 1..10 {
        let tuple = knots[i - 1];
        let hx = tuple.0;
        let hy = tuple.1;
        let tuple_t = knots[i];
        let mut tx = tuple_t.0;
        let mut ty = tuple_t.1;

        if !(touching(hx, hy, tx, ty)) {
            if hx == tx {
                sign_x = 0;
            } else {
                sign_x = (hx - tx) / (hx - tx).abs();
            }

            if hy == ty {
                sign_y = 0;
            } else {
                sign_y = (hy - ty) / (hy - ty).abs();
            }

            tx += sign_x;
            ty += sign_y;
        }
        knots[i] = (tx, ty);
    }

    knots
}

impl Runit for AocDay09 {
    fn parse(&mut self) {
        let file = crate::read_file("2022".to_string(), "09".to_string());
        self.moves = file
            .trim()
            .split("\n")
            .map(|l| l.to_string())
            .collect::<Vec<String>>();
    }
    fn second_part(&mut self) -> String {
        let mut knots = (0..10).map(|c| (c, c)).collect::<Vec<_>>();

        let mut tail_visited = HashSet::new();
        tail_visited.insert(knots[knots.len() - 1]);

        for line in &self.moves {
            let (op, amount) = line.split_once(" ").unwrap();

            let range_amount = amount.parse::<i32>().unwrap();
            let ds = op.parse::<Moves>().unwrap();

            let ds_moves = match ds {
                Moves::Up => [0, 1],
                Moves::Down => [0, -1],
                Moves::Left => [-1, 0],
                Moves::Right => [1, 0],
            };

            for _ in 1..range_amount {
                let dx = ds_moves[0];
                let dy = ds_moves[1];
                knots = move_second(dx, dy, knots);
                tail_visited.insert(knots[knots.len() - 1]);
            }
        }
        tail_visited.len().to_string()
    }
    fn first_part(&mut self) -> String {
        let mut hx = 0;
        let mut hy = 0;
        let mut tx = 0;
        let mut ty = 0;

        let mut tail_visited = HashSet::new();
        tail_visited.insert((tx, ty));

        for line in &self.moves {
            let (op, amount) = line.split_once(" ").unwrap();

            let range_amount = amount.parse::<i32>().unwrap();
            let ds = op.parse::<Moves>().unwrap();
            let ds_moves = match ds {
                Moves::Up => [0, 1],
                Moves::Down => [0, -1],
                Moves::Left => [-1, 0],
                Moves::Right => [1, 0],
            };

            for _ in 0..range_amount {
                hx += ds_moves[0];
                hy += ds_moves[1];
                let moves_made = move_it(hx, hy, tx, ty);
                tx += moves_made.0;
                ty += moves_made.1;
                tail_visited.insert((tx, ty));
            }
        }
        tail_visited.len().to_string()
    }
}
