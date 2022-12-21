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
    // Hashmap key: monkey_name, value: operation or value <- Value { i32, operation <- + - * / }
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

fn first_part(file: &str) -> i64 {
    let hash_parsed = parser(file);
    make_operation(&hash_parsed, &"root".to_string())

}
fn main() {
    let file = std::fs::read_to_string("./input").expect("Couldn't read input file");

    println!("Output 1: {:?}", first_part(&file));
}
