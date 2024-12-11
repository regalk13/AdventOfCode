use crate::Runit;

#[derive(Default)]
pub struct AocDay09 {
    input: String,
}

impl AocDay09 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl AocDay09 {
    fn solve(&self, mut files: Vec<(usize, i32)>) -> usize {
        let mut i = files.len() - 1;
        while i > 0 {
            let (size, id) = files[i];
            if id == -1 {
                i -= 1;
                continue;
            }
            if let Some(j) = files[0..i]
                .iter()
                .position(|&(s, id)| id == -1 && size <= s)
            {
                let s = files[j].0;
                files[j] = (size, id);
                files[i] = (size, -1);
                if size < s {
                    files.insert(j + 1, (s - size, -1));
                }
            }
            i -= 1;
        }
        files
            .iter()
            .flat_map(|&(s, id)| (0..s).map(move |_| id))
            .enumerate()
            .map(|(i, id)| if id == -1 { 0 } else { i * id as usize })
            .sum()
    }

    fn previous_part1(&self) -> usize {
        let mut fs1 = Vec::new();
        let mut fs2 = Vec::new();
        let mut fid = 0;
        for (i, b) in self.input.bytes().enumerate() {
            let v = if i % 2 == 0 {
                fid += 1;
                fid - 1
            } else {
                -1
            };
            fs1.extend((0..b - b'0').map(|_| (1, v)));
            fs2.push(((b - b'0') as usize, v));
        }
        let p1 = self.solve(fs1);
        p1
    }

    fn previous_part2(&self) -> usize {
        let mut fs2 = Vec::new();
        let mut fid = 0;
        for (i, b) in self.input.bytes().enumerate() {
            let v = if i % 2 == 0 {
                fid += 1;
                fid - 1
            } else {
                -1
            };
            fs2.push(((b - b'0') as usize, v));
        }
        let p2 = self.solve(fs2);
        p2
    }
}

impl Runit for AocDay09 {
    fn parse(&mut self) {
        self.input = crate::read_file("2024".to_string(), "09".to_string());
    }

    fn first_part(&mut self) -> String {
        println!("{:?}", self.input);

        self.previous_part1().to_string()
    }

    fn second_part(&mut self) -> String {
        self.previous_part2().to_string()
    }
}
