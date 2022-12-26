#[derive(Default, Debug)]
struct BluePrint {
    id: u32,
    robots: Vec<Robot>,
    geodes: u32,
}

#[derive(Debug, Default, Clone)]
struct Robot {
    type_name: String,
    costs: Vec<Material>,
}

#[derive(Default, Debug, Clone)]
struct Material {
    name: String,
    number: u32,
    max_sprend: u32,
}

fn parser(file: &str) -> Vec<BluePrint>  {
    let mut robots_vect: Vec<Robot> = vec![];
    let mut blueprints: Vec<BluePrint> = vec![];
    for line in file.trim().split("\n").collect::<Vec<&str>>() {
	let mut blueprint_struct = BluePrint::default();
	let (blueprint, robots) = line.split_once(":").unwrap();

	let blueprint_id = blueprint.split(" ").collect::<Vec<&str>>()[1];
	blueprint_struct.id = blueprint_id.parse::<u32>().unwrap();

	let robots_vec = robots.split(".").collect::<Vec<&str>>();
	for i in robots_vec {
	    let robot_data = i.trim().split(" ").collect::<Vec<&str>>();
	    if robot_data.len() <= 1 {
		continue;
	    }
	    
	    let mut costs: Vec<Material> = vec![];
	    for (n, j) in robot_data[4..].iter().enumerate() {
		// println!("Robot_data: {:?}", j);
		let mut material = Material::default();
		if *j == "and" {
		    continue
		}

		if j.len() <= 1 {
		    material.number = j.parse::<u32>().unwrap();
		    material.name = robot_data[n+4+1].to_string();
		    costs.push(material);
		}
	    }
	    let mut robot_struct = Robot::default();
	    robot_struct.type_name = robot_data[1].to_string();
	    robot_struct.costs = costs;
	    robots_vect.push(robot_struct);
	}

	blueprint_struct.robots = robots_vect.clone();
	blueprints.push(blueprint_struct);	
    }
    
    blueprints
}

fn first_part(file: &str) {
    let blue_prints = parser(file); 
    for blue_print in blue_prints {
	println!("Blueprint: {:?}", blue_print);
	println!("------");	
    }    
}


fn main() {
    let file = std::fs::read_to_string("./input.test").expect("Expected file");

    first_part(&file);
}
