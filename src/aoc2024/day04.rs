use crate::Runit;

#[derive(Default)]
pub struct AocDay04 {
    lines: Vec<Vec<char>>,
}

impl AocDay04 {
    pub fn new() -> Self {
        Self::default()
    }
}

fn find_horizontal(grid: &[Vec<char>], pattern: &[char]) -> i64 {
    let mut ans = 0;
    let (n, p) = (grid.len(), pattern.len());
    for row in grid {
        for j in 0..=(n - p) {
            let mut all_ok = true;
            for k in 0..p {
                all_ok &= pattern[k] == row[j + k];
            }
            if all_ok {
                ans += 1;
            }
        }
    }

    ans
}

fn find_diagonal(grid: &[Vec<char>], pattern: &[char]) -> i64 {
    let mut ans = 0;
    let (n, p) = (grid.len(), pattern.len());
    for i in 0..=(n - p) {
        for j in 0..=(n - p) {
            let mut all_ok = true;
            for k in 0..p {
                all_ok &= pattern[k] == grid[i + k][j + k];
            }
            if all_ok {
                ans += 1;
            }
        }
    }

    ans
}

fn rot90(grid: &mut [Vec<char>]) {
    let n = grid.len();
    for i in 0..(n / 2) {
        for j in i..(n - i - 1) {
            let temp = grid[i][j];
            grid[i][j] = grid[j][n - 1 - i];
            grid[j][n - 1 - i] = grid[n - 1 - i][n - 1 - j];
            grid[n - 1 - i][n - 1 - j] = grid[n - 1 - j][i];
            grid[n - 1 - j][i] = temp;
        }
    }
}

fn find_crosses(grid: &[Vec<char>], pattern: &[char]) -> i64 {
    let mut ans = 0;
    let (n, p) = (grid.len(), pattern.len());
    for i in 0..=(n - p) {
        for j in 0..=(n - p) {
            let mut all_ok = true;
            for k in 0..p {
                all_ok &= pattern[k] == grid[i + k][j + k];
            }
            for k in 0..p {
                all_ok &= pattern[k] == grid[i + p - 1 - k][j + k];
            }
            if all_ok {
                ans += 1;
            }
        }
    }

    ans
}

impl Runit for AocDay04 {
    fn parse(&mut self) {
        let file = crate::read_file("2024".to_string(), "04".to_string());

        self.lines = file
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<Vec<_>>>();
    }

    fn first_part(&mut self) -> String {
        let mut matches = 0;
        for _ in 0..4 {
            matches += find_horizontal(&self.lines, &['X', 'M', 'A', 'S']);
            matches += find_diagonal(&self.lines, &['X', 'M', 'A', 'S']);
            rot90(&mut self.lines);
        }

        matches.to_string()
    }

    fn second_part(&mut self) -> String {
        let mut matches = 0;
        for _ in 0..4 {
            matches += find_crosses(&self.lines, &['M', 'A', 'S']);
            rot90(&mut self.lines);
        }

        matches.to_string()
    }
}
