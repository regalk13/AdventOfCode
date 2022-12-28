use crate::Runit;

use std::collections::HashMap;

#[derive(Default)]
pub struct AocDay12 {
    maze: Maze,
    file: String,
}

impl AocDay12 {
    pub fn new() -> Self {
        AocDay12::default()
    }
}

#[derive(Debug, Clone, Default)]
struct Maze {
    grid: Vec<Vec<u8>>,
    start: (usize, usize),
    end: (usize, usize),
}

fn get_points(pos: (usize, usize), width: i32, height: i32) -> Vec<(usize, usize)> {
    const DIR: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let ipos = (pos.0 as i32, pos.1 as i32);

    DIR.iter()
        .map(|d| (ipos.0 + d.0, ipos.1 + d.1))
        .filter(|pos| pos.0 >= 0 && pos.1 >= 0 && pos.0 < height && pos.1 < width)
        .map(|pos| (pos.0 as usize, pos.1 as usize))
        .collect()
}

fn find_shortest(start: (usize, usize), end: (usize, usize), grid: Vec<Vec<u8>>) -> Option<usize> {
    let mut shortest: HashMap<(usize, usize), usize> = HashMap::new();
    shortest.insert(start, 0);

    let mut to_visit = Vec::new();
    let height = grid[0].len() as i32;
    let width = grid.len() as i32;

    to_visit.extend(get_points(start, height, width));

    while let Some(loc) = to_visit.pop() {
        let cur_elevation = grid[loc.0][loc.1];
        let points = get_points(loc, height, width);
        let valid = points
            .iter()
            .filter(|pos| grid[pos.0][pos.1] + 1 >= cur_elevation)
            .map(|pos| *pos)
            .collect::<Vec<(usize, usize)>>();

        let new_path_dist = valid.iter().filter_map(|pos| shortest.get(pos)).min();

        if new_path_dist.is_none() {
            continue;
        }

        let new_path_dist = new_path_dist.unwrap() + 1;

        let cur_path_dist = shortest.entry(loc).or_insert(usize::MAX);
        if *cur_path_dist > new_path_dist {
            *cur_path_dist = new_path_dist;
            to_visit.extend(points.iter());
        }
    }

    shortest.get(&end).copied()
}

impl Runit for AocDay12 {
    fn parse(&mut self) {
        let file = crate::read_file("2022".to_string(), "12".to_string());
        let maze = file.trim().split("\n").collect::<Vec<&str>>();
        let mut maze_struct = Maze::default();
        for (row, line) in maze.iter().enumerate() {
            let mut grid_line = line.chars().map(|c| c as u8).collect::<Vec<u8>>();
            if let Some(start) = grid_line.iter().position(|p| *p == b'S') {
                grid_line[start] = b'a';
            }
            if let Some(end) = grid_line.iter().position(|p| *p == b'E') {
                maze_struct.end = (row, end);
                grid_line[end] = b'z';
            }
            maze_struct.grid.push(grid_line);
        }
        self.file = file;
        self.maze = maze_struct;
    }
    fn second_part(&mut self) -> String {
        let maze_struct = self.maze.clone();

        let mut start_points = Vec::new();

        for (row, line) in maze_struct.grid.iter().enumerate() {
            for (col, ch) in line.iter().enumerate() {
                if *ch == b'a' {
                    start_points.push((row, col));
                }
            }
        }

        start_points
            .iter()
            .filter_map(|p| find_shortest(*p, maze_struct.end, maze_struct.grid.clone()))
            .min()
            .unwrap()
            .to_string()
    }

    fn first_part(&mut self) -> String {
        let mut maze_struct = Maze::default();
        let maze = self.file.trim().lines().collect::<Vec<&str>>();
        for (row, line) in maze.iter().enumerate() {
            let mut grid_line = line.chars().map(|c| c as u8).collect::<Vec<u8>>();
            if let Some(start) = grid_line.iter().position(|p| *p == b'S') {
                maze_struct.start = (row, start);
                grid_line[start] = b'a';
            }
            if let Some(end) = grid_line.iter().position(|p| *p == b'E') {
                maze_struct.end = (row, end);
                grid_line[end] = b'z';
            }
            maze_struct.grid.push(grid_line);
        }
        let mut shortest: HashMap<(usize, usize), usize> = HashMap::new();
        shortest.insert(maze_struct.start, 0);

        let mut to_visit = Vec::new();
        let height = maze_struct.grid[0].len() as i32;
        let width = maze_struct.grid.len() as i32;

        to_visit.extend(get_points(maze_struct.start, height, width));

        while let Some(loc) = to_visit.pop() {
            let cur_elevation = maze_struct.grid[loc.0][loc.1];
            let points = get_points(loc, height, width);
            let valid = points
                .iter()
                .filter(|pos| maze_struct.grid[pos.0][pos.1] + 1 >= cur_elevation)
                .map(|pos| *pos)
                .collect::<Vec<(usize, usize)>>();

            let new_path_dist = valid.iter().filter_map(|pos| shortest.get(pos)).min();

            if new_path_dist.is_none() {
                continue;
            }

            let new_path_dist = new_path_dist.unwrap() + 1;

            let cur_path_dist = shortest.entry(loc).or_insert(usize::MAX);
            if *cur_path_dist > new_path_dist {
                *cur_path_dist = new_path_dist;
                to_visit.extend(points.iter());
            }
        }

        shortest.get(&maze_struct.end).unwrap().to_string()
    }
}
