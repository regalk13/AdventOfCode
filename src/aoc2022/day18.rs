use crate::Runit;

use ordered_float::OrderedFloat;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Default)]
pub struct AocDay18 {
    file: String,
}
impl AocDay18 {
    pub fn new() -> Self {
        AocDay18::default()
    }
}
impl Runit for AocDay18 {
    fn parse(&mut self) {
        self.file = crate::read_file("2022".to_string(), "18".to_string());
    }

    fn first_part(&mut self) -> String {
        let mut faces: HashMap<(OrderedFloat<f32>, OrderedFloat<f32>, OrderedFloat<f32>), i32> =
            HashMap::new();
        let offsets = vec![
            (0.0, 0.0, 0.5),
            (0.0, 0.5, 0.0),
            (0.5, 0.0, 0.0),
            (0.0, 0.0, -0.5),
            (0.0, -0.5, 0.0),
            (-0.5, 0.0, 0.0),
        ];

        for line in self.file.trim().split("\n").collect::<Vec<&str>>() {
            let positions = line
                .split(",")
                .map(|l| l.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            let (x, y, z) = (positions[0], positions[1], positions[2]);

            for (dx, dy, dz) in &offsets {
                let k = (
                    OrderedFloat(x as f32 + dx),
                    OrderedFloat(y as f32 + dy),
                    OrderedFloat(dz + z as f32),
                );

                if !(faces.contains_key(&k)) {
                    faces.insert(k, 0);
                }
                if let Some(value) = faces.get_mut(&k) {
                    *value += 1;
                }
            }
        }
        let output: i32 = faces
            .iter()
            .map(|(_, v)| {
                if *v == 1 {
                    return *v;
                } else {
                    return 0 as i32;
                }
            })
            .sum();
        output.to_string()
    }

    fn second_part(&mut self) -> String {
        let mut faces: HashMap<(OrderedFloat<f32>, OrderedFloat<f32>, OrderedFloat<f32>), i32> =
            HashMap::new();
        let mut droplet = HashSet::new();
        let offsets = vec![
            (0.0, 0.0, 0.5),
            (0.0, 0.5, 0.0),
            (0.5, 0.0, 0.0),
            (0.0, 0.0, -0.5),
            (0.0, -0.5, 0.0),
            (-0.5, 0.0, 0.0),
        ];

        let (mut mx, mut my, mut mz) = (f32::INFINITY, f32::INFINITY, f32::INFINITY);
        let (mut max, mut maxy, mut maxz) =
            (f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);

        for line in self.file.trim().split("\n").collect::<Vec<&str>>() {
            let positions = line
                .split(",")
                .map(|l| l.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            let (x, y, z) = (
                positions[0] as f32,
                positions[1] as f32,
                positions[2] as f32,
            );
            droplet.insert((OrderedFloat(x), OrderedFloat(y), OrderedFloat(z)));
            mx = mx.min(x);
            my = my.min(y);
            mz = mz.min(z);

            max = max.max(x);
            maxy = maxy.max(y);
            maxz = maxz.max(z);

            for (dx, dy, dz) in &offsets {
                let k = (
                    OrderedFloat(x as f32 + dx),
                    OrderedFloat(y as f32 + dy),
                    OrderedFloat(dz + z as f32),
                );

                if !(faces.contains_key(&k)) {
                    faces.insert(k, 0);
                }
                if let Some(value) = faces.get_mut(&k) {
                    *value += 1;
                }
            }
        }

        mx -= 1.0;
        my -= 1.0;
        mz -= 1.0;

        max += 1.0;
        maxy += 1.0;
        maxz += 1.0;

        let mut q = VecDeque::from([(mx, my, mz)]);

        let mut air = HashSet::new();
        air.insert((OrderedFloat(mx), OrderedFloat(my), OrderedFloat(mz)));

        while !(q.is_empty()) {
            let (x, y, z) = q.pop_front().unwrap();
            for (dx, dy, dz) in &offsets {
                let key = (
                    OrderedFloat(x + dx * 2.0),
                    OrderedFloat(y + dy * 2.0),
                    OrderedFloat(z + dz * 2.0),
                );
                let k = (x + dx * 2.0, y + dy * 2.0, z + dz * 2.0);
                let (nx, ny, nz) = key;

                if !(mx <= *nx
                    && nx <= OrderedFloat(max)
                    && my <= *ny
                    && ny <= OrderedFloat(maxy)
                    && mz <= *nz
                    && nz <= OrderedFloat(maxz))
                {
                    continue;
                }

                if droplet.contains(&key) || air.contains(&key) {
                    continue;
                }

                air.insert(key);
                q.push_back(k);
            }
        }

        let mut free = HashSet::new();

        for (x, y, z) in air {
            for (dx, dy, dz) in &offsets {
                free.insert((
                    x + OrderedFloat(*dx),
                    y + OrderedFloat(*dy),
                    OrderedFloat(*dz) + z,
                ));
            }
        }

        let mut total = 0;

        for (key, _) in &faces {
            if free.contains(&key) {
                total += 1;
            }
        }
        total.to_string()
    }
}
