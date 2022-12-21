use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Operation {
    Sum(String, String),
    Sub(String, String),
    Mtp(String, String),
    Div(String, String),
}

impl Operation {
    fn new(value: &str) -> Self {
        if value.contains("+") {
            let (op1, op2) = value.split_once("+").unwrap();
            return Operation::Sum(op1.to_string(), op2.to_string());
        } else if value.contains("-") {
            let (op1, op2) = value.split_once("-").unwrap();
            return Operation::Sub(op1.to_string(), op2.to_string());
        } else if value.contains("/") {
            let (op1, op2) = value.split_once("/").unwrap();
            return Operation::Div(op1.to_string(), op2.to_string());
        } else {
            let (op1, op2) = value.split_once("*").unwrap();
            return Operation::Mtp(op1.to_string(), op2.to_string());
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Value {
    Integer(i32),
    Op(Operation),
}

impl Value {
    fn new(value: &str) -> Self {
        if value.len() <= 3 {
            return Value::Integer(value.parse::<i32>().unwrap());
        } else {
            return Value::Op(Operation::new(value));
        }
    }

}

fn make_operation(op: Value, op2: Value) {

}

fn parser(file: &str) -> HashMap<String, Value> {
    // Hashmap key: monkey_name, value: operation or value <- Value { i32, operation <- + - * / }
    file.trim()
        .lines()
        .map(|line| {
            let (name, value) = line.split_once(": ").unwrap();
            (name.to_string(), Value::new(value.trim()))
        })
        .collect::<HashMap<String, Value>>()
}

fn search_value(hash: HashMap<String, Value>, key: String) -> Value {
   let value = &hash[&key];
    match value {
        Value::Integer(_) => {
            value.clone()
        }
        Value::Op(_) => {
            /*
            if let Value::Op(value) = value {
                value.make_operation();
            }*/
            value.clone()
        }
    }
}

fn first_part(file: &str) {
    let hash_parsed = parser(file);
    let output = search_value(hash_parsed, "root".to_string());

}
fn main() {
    let file = std::fs::read_to_string("./input.test").expect("Couldn't read input file");

    first_part(&file);
}
