use crate::Runit;
use hashbrown::HashMap;
use std::collections::HashSet;

#[derive(Default)]
pub struct AocDay08 {
    grid: Vec<Vec<char>>,
    col_size: usize,
    row_size: usize,
    antennas: HashMap<char, Vec<(i32, i32)>>,
}

impl AocDay08 {
    pub fn new() -> Self {
        Self::default()
    }
    fn in_bounds(&self, pos: (i32, i32)) -> bool {
        (0..self.col_size as i32).contains(&pos.0) && (0..self.row_size as i32).contains(&pos.1)
    }
}

impl Runit for AocDay08 {
    fn parse(&mut self) {
        let file = crate::read_file("2024".to_string(), "08".to_string());

        self.grid = file
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect();
        self.row_size = self.grid.len();
        self.col_size = self.grid[0].len();

        self.antennas = self
            .grid
            .iter()
            .enumerate()
            .flat_map(|(row_index, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, &cell)| cell != '.')
                    .map(move |(col_index, &cell)| (cell, (row_index as i32, col_index as i32)))
            })
            .fold(HashMap::new(), |mut acc, (cell, pos)| {
                acc.entry(cell).or_insert_with(Vec::new).push(pos);
                acc
            });
    }

    fn first_part(&mut self) -> String {
        let mut antinodes = HashSet::new();

        for positions in self.antennas.values() {
            for (i, &(r1, c1)) in positions.iter().enumerate() {
                for &(r2, c2) in &positions[i + 1..] {
                    let pos1 = (2 * r1 - r2, 2 * c1 - c2);
                    let pos2 = (2 * r2 - r1, 2 * c2 - c1);

                    if self.in_bounds(pos1) {
                        antinodes.insert(pos1);
                    }
                    if self.in_bounds(pos2) {
                        antinodes.insert(pos2);
                    }
                }
            }
        }

        antinodes.len().to_string()
    }

    fn second_part(&mut self) -> String {
        let mut antinodes = HashSet::new();

        for positions in self.antennas.values() {
            for &(r1, c1) in positions {
                for &(r2, c2) in positions {
                    if (r1, c1) == (r2, c2) {
                        continue;
                    }

                    let dr = r2 - r1;
                    let dc = c2 - c1;

                    let mut r = r1 + dr;
                    let mut c = c1 + dc;

                    while self.in_bounds((r, c)) {
                        antinodes.insert((r, c));
                        r += dr;
                        c += dc;
                    }
                }
            }
        }

        antinodes.len().to_string()
    }
}
