use std::collections::VecDeque;

use hashbrown::HashSet;

use crate::Runit;

#[derive(Default)]
pub struct AocDay10 {
    pos: Vec<Vec<i32>>,
    trailheads: Vec<(usize, usize)>,
    rows: usize,
    cols: usize,
}

impl AocDay10 {
    pub fn new() -> Self {
        Self::default()
    }

    fn solve_pt1(&mut self, r: i32, c: i32, pt2: bool) -> usize {
        let mut deque: VecDeque<(i32, i32)> = VecDeque::new();
        let mut summits: HashSet<(i32, i32)> = HashSet::new();
        let mut summits2: usize = 0;
        deque.push_back((r, c));

        while let Some((cr, cc)) = deque.pop_front() {
            for (nr, nc) in [(cr, cc + 1), (cr, cc - 1), (cr + 1, cc), (cr - 1, cc)] {
                if nr < 0 || nc < 0 || nr >= self.rows as i32 || nc >= self.cols as i32 {
                    continue;
                }

                let (nr_usize, nc_usize) = (nr as usize, nc as usize);

                if self.pos[nr_usize][nc_usize] != self.pos[cr as usize][cc as usize] + 1 {
                    continue;
                }

                if self.pos[nr_usize][nc_usize] == 9 {
                    summits.insert((nr, nc));
                    summits2 += 1;
                } else {
                    deque.push_back((nr, nc));
                }
            }
        }

        if pt2 {
            summits2
        } else {
            summits.len()
        }
    }
}

impl Runit for AocDay10 {
    fn parse(&mut self) {
        let file = crate::read_file("2024".to_string(), "10".to_string());

        self.pos = file
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).expect("Invalid character") as i32) // Convertir a i32
                    .collect()
            })
            .collect();

        self.trailheads = self
            .pos
            .iter()
            .enumerate()
            .flat_map(|(row_idx, row)| {
                row.iter().enumerate().filter_map(move |(col_idx, &val)| {
                    if val == 0 {
                        Some((row_idx, col_idx))
                    } else {
                        None
                    }
                })
            })
            .collect();

        self.rows = self.pos.len();
        self.cols = self.pos[0].len();
    }
    fn first_part(&mut self) -> String {
        let result = self
            .trailheads
            .clone()
            .into_iter()
            .map(|(r, c)| self.solve_pt1(r as i32, c as i32, false))
            .sum::<usize>();
        result.to_string()
    }

    fn second_part(&mut self) -> String {
        let result = self
            .trailheads
            .clone()
            .into_iter()
            .map(|(r, c)| self.solve_pt1(r as i32, c as i32, true))
            .sum::<usize>();

        result.to_string()
    }
}
