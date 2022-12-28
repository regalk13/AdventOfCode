use crate::Runit;

use num::integer::Integer;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Default)]
pub struct AocDay24 {
    map: Map,
}

impl AocDay24 {
    pub fn new() -> Self {
        AocDay24::default()
    }
}

// Posisition struct
// can't use complex numbers cause module operations
#[derive(Copy, Clone, Default)]
struct Pos {
    x: usize,
    y: usize,
}

// Builder pos struct
impl Pos {
    fn new(x: usize, y: usize) -> Pos {
        Pos { x, y }
    }
}

type Bitmask = u128;

// Creathe the cell of posible moves
#[derive(Eq, PartialEq)]
struct Cell {
    n: Bitmask,
    s: Bitmask,
    e: Bitmask,
    w: Bitmask,
}

// Position and define walls
impl Cell {
    fn wall() -> Cell {
        Cell {
            n: Bitmask::MAX,
            s: Bitmask::MAX,
            e: Bitmask::MAX,
            w: Bitmask::MAX,
        }
    }
    // don't move
    fn empty() -> Cell {
        Cell {
            n: 0,
            s: 0,
            e: 0,
            w: 0,
        }
    }
    // move up
    fn north() -> Cell {
        Cell {
            n: 1,
            s: 0,
            e: 0,
            w: 0,
        }
    }
    // move down
    fn south() -> Cell {
        Cell {
            n: 0,
            s: 1,
            e: 0,
            w: 0,
        }
    }
    // move right
    fn east() -> Cell {
        Cell {
            n: 0,
            s: 0,
            e: 1,
            w: 0,
        }
    }
    // move left
    fn west() -> Cell {
        Cell {
            n: 0,
            s: 0,
            e: 0,
            w: 1,
        }
    }
}

// Definin the map
#[derive(Default)]
struct Map {
    width: usize,
    height: usize,
    start: Pos,
    finish: Pos,
    grid: Vec<Cell>,
}

// implement the map
impl Map {
    // parsing the input  in a grid
    pub fn parse(input: &str) -> Map {
        let lines: Vec<&str> = input.lines().collect();
        let first = lines.first().unwrap();
        let last = lines.last().unwrap();
        let width = first.len();
        let height = lines.len();
        let start_x = first.chars().position(|c| c == '.').unwrap();
        let finish_x = last.chars().position(|c| c == '.').unwrap();
        let mut grid = Vec::with_capacity(width * height);
        for (y, line) in lines.iter().enumerate() {
            assert_eq!(line.len(), width);
            for (x, c) in line.chars().enumerate() {
                grid.push(match c {
                    '#' => {
                        assert!(x == 0 || y == 0 || x == width - 1 || y == height - 1);
                        Cell::wall()
                    }
                    '.' => Cell::empty(),
                    '^' => Cell::north(),
                    'v' => Cell::south(),
                    '<' => Cell::west(),
                    '>' => Cell::east(),
                    _ => unreachable!(),
                });
            }
        }
        let mut map = Map {
            width,
            height,
            start: Pos::new(start_x, 0),
            finish: Pos::new(finish_x, height - 1),
            grid,
        };
        map.propagate_winds();
        map
    }
    // creating the cells
    fn cell(&self, x: usize, y: usize) -> Option<&Cell> {
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(&self.grid[y * self.width + x])
    }

    fn cell_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(&mut self.grid[y * self.width + x])
    }

    fn cell_is_wall(&self, x: usize, y: usize) -> bool {
        let wall = Cell::wall();
        let cell = self.cell(x, y).unwrap_or(&wall);
        *cell == wall
    }

    fn longest_cycle(&self) -> usize {
        let e_w_cycle = self.width - 2;
        let n_s_cycle = self.height - 2;
        e_w_cycle.lcm(&n_s_cycle)
    }

    fn cell_is_safe(&self, x: usize, y: usize, t: usize) -> bool {
        let e_w_cycle = self.width - 2;
        let n_s_cycle = self.height - 2;
        let e_w_t = t % e_w_cycle;
        let n_s_t = t % n_s_cycle;
        let wall = Cell::wall();
        let cell = self.cell(x, y).unwrap_or(&wall);
        (cell.e >> e_w_t) & 1 == 0
            && (cell.w >> e_w_t) & 1 == 0
            && (cell.n >> n_s_t) & 1 == 0
            && (cell.s >> n_s_t) & 1 == 0
    }
    // blizzards moving each time
    fn propagate_winds(&mut self) {
        let e_w_cycle = self.width - 2;
        let n_s_cycle = self.height - 2;
        for y in 1..self.height - 1 {
            for x in 1..self.width - 1 {
                for t in 1..e_w_cycle {
                    let w_wind = 1 + (x - 1 + t) % e_w_cycle;
                    let e_wind = 1 + (x - 1 + e_w_cycle - t) % e_w_cycle;
                    self.cell_mut(x, y).unwrap().e |= (self.cell(e_wind, y).unwrap().e & 1) << t;
                    self.cell_mut(x, y).unwrap().w |= (self.cell(w_wind, y).unwrap().w & 1) << t;
                }
                for t in 1..n_s_cycle {
                    let s_wind = 1 + (y - 1 + n_s_cycle - t) % n_s_cycle;
                    let n_wind = 1 + (y - 1 + t) % n_s_cycle;
                    self.cell_mut(x, y).unwrap().n |= (self.cell(x, n_wind).unwrap().n & 1) << t;
                    self.cell_mut(x, y).unwrap().s |= (self.cell(x, s_wind).unwrap().s & 1) << t;
                }
            }
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    x: usize,
    y: usize,
    cycle: usize,
    t: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .t
            .cmp(&self.t)
            .then_with(|| self.x.cmp(&other.x))
            .then_with(|| self.y.cmp(&other.y))
            .then_with(|| self.cycle.cmp(&other.cycle))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// dijkstra function to find the best moves to reach the end in less time
fn dijkstra(map: &Map, start: Pos, start_time: usize, finish: Pos) -> usize {
    let mut earliest_time = HashMap::new();
    let mut queue = BinaryHeap::new();
    let longest_cycle = map.longest_cycle();
    earliest_time.insert((start.x, start.y, start_time % longest_cycle), start_time);
    queue.push(State {
        x: start.x,
        y: start.y,
        cycle: start_time % longest_cycle,
        t: start_time,
    });
    while let Some(State {
        x: ux,
        y: uy,
        cycle,
        t: ut,
    }) = queue.pop()
    {
        if ut > earliest_time[&(ux, uy, cycle)] {
            continue;
        }
        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let vx = ux.wrapping_add_signed(dx);
            let vy = uy.wrapping_add_signed(dy);
            if (vx, vy) == (finish.x, finish.y) {
                let vt = ut + 1;
                earliest_time.insert((vx, vy, vt % longest_cycle), vt);
                return vt;
            }
            if map.cell_is_wall(vx, vy) {
                continue;
            }
            for vt in ut + 1..ut + longest_cycle + 1 {
                if map.cell_is_safe(vx, vy, vt) {
                    if vt
                        < *earliest_time
                            .get(&(vx, vy, vt % longest_cycle))
                            .unwrap_or(&usize::MAX)
                    {
                        earliest_time.insert((vx, vy, vt % longest_cycle), vt);
                        queue.push(State {
                            x: vx,
                            y: vy,
                            cycle: vt % longest_cycle,
                            t: vt,
                        });
                    }
                    break;
                }
                if !map.cell_is_safe(ux, uy, vt) {
                    break;
                }
            }
        }
    }
    unreachable!()
}

impl Runit for AocDay24 {
    fn parse(&mut self) {
        let input = crate::read_file("2022".to_string(), "24".to_string());
        self.map = Map::parse(&input);
    }

    fn first_part(&mut self) -> String {
        dijkstra(&self.map, self.map.start, 0, self.map.finish).to_string()
    }

    // two searchs dijsktra to get back to the start and then go back to the final position
    fn second_part(&mut self) -> String {
        let t1 = dijkstra(&self.map, self.map.start, 0, self.map.finish);
        let t2 = dijkstra(&self.map, self.map.finish, t1, self.map.start);
        dijkstra(&self.map, self.map.start, t2, self.map.finish).to_string()
    }
}
