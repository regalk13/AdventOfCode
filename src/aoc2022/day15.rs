fn dist(a: (i64, i64), b: (i64, i64)) -> i64 {
    return (a.0 - b.0).abs() + (a.1 - b.1).abs();
}

fn second_part(file: &str) {
    let mut sensors = vec![];
    let mut beacons = vec![];
    for line in file.trim().split("\n").collect::<Vec<&str>>() {
        let parts = line.split(" ").collect::<Vec<&str>>();

        let sx_vec = parts[2].split("=").collect::<Vec<&str>>();
        let sy_vec = parts[3].split("=").collect::<Vec<&str>>();
        let bx_vec = parts[8].split("=").collect::<Vec<&str>>();
        let by_vec = parts[9].split("=").collect::<Vec<&str>>();

        let sx: i64 = sx_vec[1].replace(",", "").parse().unwrap();
        let sy: i64 = sy_vec[1].replace(":", "").parse().unwrap();
        let bx: i64 = bx_vec[1].replace(",", "").parse().unwrap();
        let by: i64 = by_vec[1].parse().unwrap();
        sensors.push((sx, sy));
        beacons.push((bx, by));
    }
    let n = sensors.len();
    let mut dists = vec![];

    for i in 0..n {
        dists.push(dist(sensors[i], beacons[i]))
    }

    let mut post_lines = vec![];
    let mut neg_lines = vec![];

    for (i, s) in sensors.iter().enumerate() {
        let d = dists[i];
        neg_lines.extend([s.0 + s.1 - d, s.0 + s.1 + d]);
        post_lines.extend([s.0 - s.1 - d, s.0 - s.1 + d]);
    }
    let mut pos = 0;
    let mut neg = 0;

    for i in 0..2 * n {
        for j in i + 1..2 * n {
            let a = post_lines[i];
            let b = post_lines[j];

            if (a - b).abs() == 2 {
                pos = a.min(b) + 1;
            }

            let a = neg_lines[i];
            let b = neg_lines[j];

            if (a - b).abs() == 2 {
                neg = a.min(b) + 1;
            }
        }
    }

    let x = (pos + neg) / 2;
    let y = (neg - pos) / 2;

    let output = x * 4_000_000 + y;
    println!("Output 2: {}", output);
}

fn first_part(file: &str) {
    let mut sensors = vec![];
    let mut beacons = vec![];
    for line in file.trim().split("\n").collect::<Vec<&str>>() {
        let parts = line.split(" ").collect::<Vec<&str>>();

        let sx_vec = parts[2].split("=").collect::<Vec<&str>>();
        let sy_vec = parts[3].split("=").collect::<Vec<&str>>();
        let bx_vec = parts[8].split("=").collect::<Vec<&str>>();
        let by_vec = parts[9].split("=").collect::<Vec<&str>>();

        let sx: i64 = sx_vec[1].replace(",", "").parse().unwrap();
        let sy: i64 = sy_vec[1].replace(":", "").parse().unwrap();
        let bx: i64 = bx_vec[1].replace(",", "").parse().unwrap();
        let by: i64 = by_vec[1].parse().unwrap();
        sensors.push((sx, sy));
        beacons.push((bx, by));
    }
    let n = sensors.len();
    let mut dists = vec![];

    for i in 0..n {
        dists.push(dist(sensors[i], beacons[i]))
    }

    let y = 2_000_000;

    let mut intervals = vec![];

    for (i, s) in sensors.iter().enumerate() {
        let dx = dists[i] - (s.1 - y).abs();

        if dx <= 0 {
            continue;
        }

        intervals.push((s.0 - dx, s.0 + dx));
    }

    let mut allowed_x = vec![];

    for (bx, by) in beacons {
        if by == y {
            allowed_x.push(bx);
        }
    }

    println!("Allowed {:?}", allowed_x);

    let min_x = intervals.iter().map(|i| i.0).min().unwrap();
    let max_x = intervals.iter().map(|i| i.1).max().unwrap();

    let mut output = 0;

    for x in min_x..max_x + 1 {
        if allowed_x.contains(&x) {
            continue;
        }

        for (left, right) in &intervals {
            if *left <= x && x <= *right {
                output += 1;
                break;
            }
        }
    }

    println!("Output: {}", output);
}

fn main() {
    let file = std::fs::read_to_string("./input").expect("Expected file");

    first_part(&file);
    second_part(&file);
}
