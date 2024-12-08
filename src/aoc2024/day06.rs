use crate::Runit;

#[derive(Default)]
pub struct AocDay06 {
    lines: Vec<Vec<char>>,
}

impl AocDay06 {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

impl AocDay06 {
    fn is_guard_in_loop(
        &self,
        mut guard_location: (i32, i32),
        modified_lines: &Vec<Vec<char>>,
    ) -> bool {
        let mut direction = Direction::Up;
        let mut seen = std::collections::HashSet::new();

        loop {
            let (mut factor_direction_y, mut factor_direction_x) = (0, 0);

            match direction {
                Direction::Up => factor_direction_y = -1,
                Direction::Down => factor_direction_y = 1,
                Direction::Left => factor_direction_x = -1,
                Direction::Right => factor_direction_x = 1,
            }

            let next_position = (
                guard_location.0 + factor_direction_y,
                guard_location.1 + factor_direction_x,
            );

            if !self.in_bounds(next_position) {
                return false;
            }

            let next_char = modified_lines[next_position.0 as usize][next_position.1 as usize];

            if next_char != '#' {
                guard_location = next_position;

                if seen.contains(&(guard_location, direction.clone())) {
                    return true;
                }
                seen.insert((guard_location, direction.clone()));
            } else {
                direction = match direction {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                };
            }
        }
    }

    fn find_guard(&self) -> (i32, i32) {
        for (i, line) in self.lines.iter().enumerate() {
            for (l, &char) in line.iter().enumerate() {
                if char == '^' {
                    return (i as i32, l as i32);
                }
            }
        }
        (0, 0)
    }

    fn in_bounds(&self, position: (i32, i32)) -> bool {
        position.0 >= 0
            && position.1 >= 0
            && position.0 < self.lines.len() as i32
            && position.1 < self.lines[0].len() as i32
    }

    fn trace_path(
        &self,
        mut guard_location: (i32, i32),
        mut direction: Direction,
    ) -> Vec<(i32, i32)> {
        let mut positions = std::collections::HashSet::new();

        loop {
            let (mut factor_direction_y, mut factor_direction_x) = (0, 0);

            match direction {
                Direction::Up => factor_direction_y = -1,
                Direction::Down => factor_direction_y = 1,
                Direction::Left => factor_direction_x = -1,
                Direction::Right => factor_direction_x = 1,
            }

            let next_position = (
                guard_location.0 + factor_direction_y,
                guard_location.1 + factor_direction_x,
            );

            if !self.in_bounds(next_position) {
                break;
            }

            let next_char = self.lines[next_position.0 as usize][next_position.1 as usize];

            if next_char != '#' {
                guard_location = next_position;
                positions.insert(guard_location);
            } else {
                direction = match direction {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                };
            }
        }

        positions.into_iter().collect()
    }
}

impl Runit for AocDay06 {
    fn parse(&mut self) {
        let file = crate::read_file("2024".to_string(), "06".to_string());

        self.lines = file
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<Vec<_>>>();
    }

    fn first_part(&mut self) -> String {
        let guard_location = self.find_guard();
        let positions = self.trace_path(guard_location, Direction::Up);
        println!("{:?}", positions);
        positions.len().to_string()
    }

    fn second_part(&mut self) -> String {
        let guard_location = self.find_guard();

        let mut positions = Vec::new();

        let valid_positions: Vec<(i32, i32)> = self.trace_path(guard_location, Direction::Up);

        for position in valid_positions {
            self.lines[position.0 as usize][position.1 as usize] = '#';

            if self.is_guard_in_loop(guard_location, &self.lines) {
                positions.push(position);
            }
            self.lines[position.0 as usize][position.1 as usize] = '.';
        }

        positions.len().to_string()
    }
}
