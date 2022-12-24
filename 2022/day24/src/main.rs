use num::integer::Integer;
use std::collections::{HashSet, VecDeque};

struct Valley {
    blizzards: Vec<HashSet<(i32, i32)>>,
    r: i32,
    c: i32,
}

impl Valley {
    fn new(file: &str) -> Self {
        let mut blizzarrds: Vec<HashSet<(i32, i32)>> = [
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
        ]
        .to_vec();
        let mut r_ = 0;
        let mut c_ = 0;
        for (r, line) in file.trim().lines().collect::<Vec<&str>>()[1..]
            .iter()
            .enumerate()
        {
            for (c, item) in line.chars().collect::<Vec<char>>()[1..].iter().enumerate() {
                match item {
                    '<' => blizzarrds[0].insert((r as i32, c as i32)),
                    '>' => blizzarrds[1].insert((r as i32, c as i32)),
                    '^' => blizzarrds[2].insert((r as i32, c as i32)),
                    'v' => blizzarrds[3].insert((r as i32, c as i32)),
                    _ => false,
                };
                c_ = c;
            }
            r_ = r;
        }
        Valley {
            blizzards: blizzarrds,
            r: r_ as i32,
            c: c_ as i32,
        }
    }
}

fn first_part(file: &str) {
    let valley = Valley::new(file);
    let mut queue = VecDeque::from([(0, -1, 0)]);
    let mut seen = HashSet::new();
    let (r, c) = (valley.r, valley.c);
    let target = (r, c - 1);
    let lcm = r * c / r.gcd(&c);
    while queue.len() > 0 {
        let current = queue.pop_front().unwrap();
        let mut time = current.0;
        let cr = current.1;
        let cc = current.2;    
        time += 1;
        
        for (dr, dc) in [(0, 1), (0, -1), (-1, 0), (1, 0), (0, 0)] {
            let mut pass = true;
            let nr = cr + dr;
            let nc = cc + dc;
            if (nr, nc) == target {
                println!("time: {:?}", time);
            }
        
           if (nr < 0 || nc < 0 || nr >= r || nc >= c) && !((nr, nc) == (-1, 0)) {
                continue;
            }
       
            for (i, tr, tc) in [(0, 0, -1), (1, 0, 1), (2, -1, 0), (3, 1, 0)] {
                let contained = ((nr - tr * time) % r, (nc - tc * time) % c);
                if valley.blizzards[i].contains(&contained) {
                    pass = false;
                    break;
                }
            }
            if pass == true {
                let key = (nr, nc, time % lcm);

                if seen.contains(&key) {
                    continue;
                }

                seen.insert(key);

                queue.push_back((time, nr, nc));
            }
        }
    }
}

fn main() {
    let file = std::fs::read_to_string("./input").expect("couldn't open input file :(");

    first_part(&file);

    println!("{}", file);
}
