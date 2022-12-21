use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Operation {
    Add,
    Sub,
    Mult,
    Div
}

impl Operation {
    fn parse(ch: char) -> Self {
        match ch {
            '+' => Self::Add,
            '-' => Self::Sub,
            '*' => Self::Mult,
            '/'=> Self::Div,
            _ => panic!("Invalid operation: {}", ch),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Value {
    Integer(i64),
    Op(String, Operation, String),
}

fn parser(file: &str) -> HashMap<String, Value> {
    // Hashmap key: monkey_name, value: operation or value <- Value { i64, operation <- + - * / }
    file.trim()
        .lines()
        .map(|line| {
            let (name, value) = line.split_once(": ").unwrap();
            
            if let Ok(num) = value.parse::<i64>()
            {
                (name.to_string(), Value::Integer(num))
            } else 
            {
                let expr = value.split_whitespace().collect::<Vec<&str>>();
                (name.to_string(), Value::Op(expr[0].to_string(), Operation::parse(expr[1].chars().next().unwrap()), expr[2].to_string()))
            }

        })
        .collect::<HashMap<String, Value>>()
}

fn make_operation(hash: &HashMap<String, Value>, key: &String) -> i64 {
   let value = hash.get(key).unwrap();
    match value {
        Value::Integer(v) => *v,
        Value::Op(l, op, r) => {
            let l = make_operation(hash, l);
            let r = make_operation(hash, r);
            match op {
                Operation::Add => l + r,
                Operation::Sub => l - r,
                Operation::Mult => l * r,
                Operation::Div => l / r,
            }
        }
    }
}

fn find_me<'a>(loc: &'a String, tree: &'a HashMap<String, Value>) -> Option<Vec<&'a String>> {
    if loc == "humn" {
        return Some(vec![loc]);
    }
    if let Some(Value::Op(l, _, r)) =  tree.get(loc) {
        if let Some(mut vec) = find_me(l, tree) {
            vec.push(loc);
            return Some(vec);
        }
        if let Some(mut vec) = find_me(r, tree) {
            vec.push(loc);
            return Some(vec);
        }
    }

    None
}

fn find_adjustment(path: &Vec<&String>, index: usize, tree: &HashMap<String, Value>, cv: i64) -> i64 {
    match tree.get(path[index]).unwrap() {
        Value::Integer(_) => cv,
        Value::Op(l, op, r) => {
            let left = make_operation(tree, l);
            let right = make_operation(tree, r);
            let new_cv = if l == path[index +  1] {
                match op {
                    Operation::Add => cv - right,
                    Operation::Sub => cv + right,
                    Operation::Mult => cv / right,
                    Operation::Div => cv * right,
                }
            } else {
                match op {
                    Operation::Add => cv - left,
                    Operation::Sub => left - cv,
                    Operation::Mult => cv / left,
                    Operation::Div => left / cv,
                }
            };
            find_adjustment(path, index + 1, tree, new_cv)
        }
    }
}

fn second_part(hash: &HashMap<String, Value>) -> i64 {
    let root_name = "root".to_string();
    let path = find_me(&root_name, &hash).unwrap();
    let path = path.iter().rev().copied().collect::<Vec<_>>();
    
    let (left, right) = match hash.get(&root_name).unwrap() {
        Value::Integer(_) => panic!("Root without data"),
        Value::Op(left, _, right) => (left, right),
    };

    let right_num = if left == path[1] {
        make_operation(&hash, &right)
    } else {
        make_operation(&hash, &left)
    };

    find_adjustment(&path, 1, &hash, right_num)
}


fn first_part(hash: &HashMap<String, Value>) -> i64 {
    make_operation(hash, &"root".to_string())

}
fn main() {    
    let file = std::fs::read_to_string("./input").expect("Couldn't read input file");
    let parsed = parser(&file);
    println!("Output 1: {:?}", first_part(&parsed));
    println!("Output 2: {:?}", second_part(&parsed));
}
