use regex::Regex;
use std::collections::HashMap;

fn dfs(bp: Vec<Vec<(i32, i32)>>, max_spend: Vec<i32>, mut cache: HashMap<(i32, i32, i32, i32, i32, i32, i32, i32, i32), i32>, time: i32, bots: Vec<i32>, amt: Vec<i32>)  -> i32{
    if time == 0 {
	    return amt[3]
    }

    let key = (time, bots[0], bots[1], bots[2], bots[3], amt[0], amt[1], amt[2], amt[3]);

    if cache.contains_key(&key) {
	    return cache[&key];
    }

    let mut max_val = amt[3] + bots[3] * time;

    for (btype, recipe) in bp.iter().enumerate() {
	if btype != 3  && bots[btype] >= max_spend[btype] {
	    continue;
	}
	let mut wait = 0;

	for (ramt, rtype) in recipe {
	    if bots[*rtype as usize] == 0{
		    break;
	    }
	    wait = wait.max(-(-(*ramt - amt[*rtype as usize]) / bots[*rtype as usize]));
	}
	let remtime = time - wait - 1;
	if remtime <= 0 {
	    continue
	}
	let mut bots_ = bots.clone();
	let mut amt_ = std::iter::zip(amt.clone(), bots.clone()).map(|(x, y)|x + y * (wait + 1)).collect::<Vec<i32>>();
    println!("amt_: {:?}", amt_);
	for (ramt, rtype) in recipe {
	    amt_[*rtype as usize] -= ramt;
	}
	bots_[btype] += 1;
	for i in 0..3 {
	    amt_[i] = amt_[i].min(max_spend[i] * remtime);
	}
	    max_val = max_val.max(dfs(bp.clone(), max_spend.clone(), cache.clone(), remtime, bots_, amt_));
    }
    cache.insert(key, max_val);
    return max_val;
}


fn main() {
    let mut total = 0;
    let file = std::fs::read_to_string("./input.test").expect("Expected file");
    for (i, l) in file.trim().split("\n").collect::<Vec<&str>>().iter().enumerate() {
	let mut bp = vec![];
	let mut max_spend: Vec<i32> = vec![0, 0, 0];
	for section in l.split(": ").collect::<Vec<&str>>()[1].split(". ").collect::<Vec<&str>>() {
	    let regex = Regex::new(r"(\d+) (\w+)").unwrap();
	    let mut recipe = vec![];
	    for r in regex.captures_iter(section) {
		let robots = vec!["ore", "clay", "obsidian"];
		let x = r[1].parse::<i32>().unwrap();
		let y = robots.iter().position(|&x| *x == r[2]).unwrap();
		recipe.push((x,y as i32));
		max_spend[y] = x.max(max_spend[y]); 
	    }
	    bp.push(recipe);
	}

	let v = dfs(bp, max_spend, HashMap::new(), 24, [1, 0, 0, 0].to_vec(), [0, 0, 0, 0].to_vec());
	total += (i as i32 + 1) * v;
    }

    println!("Total: {}", total);
}
