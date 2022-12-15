
fn dist(a: (i32, i32), b: (i32, i32)) -> i32 {
    return (a.0 - b.0).abs() + (a.1 - b.1).abs()
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

	let sx: i32 = sx_vec[1].replace(",", "").parse().unwrap();
	let sy: i32 = sy_vec[1].replace(":","").parse().unwrap();
	let bx: i32 = bx_vec[1].replace(",","").parse().unwrap();
	let by: i32 = by_vec[1].parse().unwrap();
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
	    continue
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

    for x in min_x..max_x+1 {
	if allowed_x.contains(&x) {
	    continue
	}

	for (left, right) in &intervals {
	    if *left <= x && x <= *right {
		output += 1;
		break
	    }
	}
    }

    println!("Output: {}", output);
}

fn main() {
    let file = std::fs::read_to_string("./input").expect("Expected file");

    first_part(&file);    
}
