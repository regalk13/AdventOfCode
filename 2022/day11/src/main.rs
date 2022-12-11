#[derive(Debug, Default, Clone)]
enum Operation {
    #[default] Sum,
    Multiply
}

impl Operation {
    fn make_operation(&self, old: u64, op: u64) -> u64 {
        match self {
            Operation::Sum => old + op,
            Operation::Multiply => old * op
        }
    }

    fn make_operation_part2(&self, old: u64, op: u64, magic_trick: u64) -> u64 {
         match self {
            Operation::Sum => (old + op)%magic_trick,
            Operation::Multiply => (old * op)%magic_trick
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Monkey {
    id: i32,
    items: Vec<u64>,
    operation: Operation,
    old: u64,
    test: u64,
    trower: Vec<u64>,
    inspected: u64,
}

fn parser(monkeys: Vec<&str>) -> Vec<Monkey> {
    let mut monkey_list: Vec<Monkey> = vec![];
    for monkey in monkeys {
        let mut monkey_struct = Monkey::default();
        for m in monkey.split("\n").collect::<Vec<&str>>() {
            if m.starts_with("Monkey") {
                let (_, id) = m.split_once(" ").unwrap();
                monkey_struct.id = id.trim_end_matches(":").parse::<i32>().unwrap();
            } else if m.trim().starts_with("S") {
                let (_, items) = m.split_once(":").unwrap();

                let items = items.trim().split(", ")
                    .map(|item| {
                        item.parse::<u64>().unwrap()
                    })
                .collect::<Vec<u64>>();

                monkey_struct.items = items;
            } else if m.trim().starts_with("Op") {
                let (_, operation) = m.split_once("= ").unwrap();
                if operation.contains("*") {
                    let (_, number) = operation.split_once("* ").unwrap();
                    if number == "old" {
                        monkey_struct.old = 0;
                        monkey_struct.operation = Operation::Multiply;
                    } else {
                        monkey_struct.old = number.parse::<u64>().unwrap();
                        monkey_struct.operation = Operation::Multiply;
                    }
                } else {
                    let (_, number) = operation.split_once("+ ").unwrap();
                    monkey_struct.old = number.parse::<u64>().unwrap();
                    monkey_struct.operation = Operation::Sum;
                }
            } else if m.trim().starts_with("Te") {
                let (_, test) = m.split_once(":").unwrap();
                let (_, test) = test.split_once("by ").unwrap();
                monkey_struct.test = test.parse::<u64>().unwrap();
            } else if m.trim().starts_with("If") {
                let (_, trow) = m.split_once("monkey ").unwrap();
                monkey_struct.trower.push(trow.parse::<u64>().unwrap());
            }
        }
        //monkey_struct.old = monkey_struct.operation.make_operation(monkey_struct.items[0], monkey_struct.old); 
        monkey_list.push(monkey_struct);
    };
    monkey_list
}

fn part_2(file: &str) { 
    let monkeys = file.trim().split("\n\n").collect::<Vec<&str>>();
    let mut monkey_list = parser(monkeys);
    let magic_trick = monkey_list
        .iter()
        .map(|monkey| monkey.test)
        .product::<u64>();
    for i in 0..10_000 {
        println!("After round {}, the monkeys are holding items with these worry levels:", i);
        for (l, m) in monkey_list.clone().iter().enumerate() {
            let monkey_listed = monkey_list.clone();
            for (j, item) in monkey_listed[l].items.iter().enumerate() {
                let mut new = 0;
                if m.old == 0 {
                    new = m.operation.make_operation_part2(*item, *item, magic_trick);
                }
                else {
                    new = m.operation.make_operation_part2(*item, m.old, magic_trick);
                }
                // println!("Operation: {:?}, new: {}, old: {}", m.operation, new, m.old);
                if new % m.test == 0 {
                    monkey_list[m.trower[0] as usize].items.push(new);
                    //println!("Item with {} of worry, because: {} from monkey {} trowed to {} monkey", new, item, m.id, m.trower[0]);
                    monkey_list[l].inspected += 1;
                    monkey_list[l].items.remove(0);  
                } else { 
                    monkey_list[m.trower[1] as usize].items.push(new);
                    // println!("Item with {} of worry, because: {} from monkey {} trowed to {} monkey", new, item, m.id, m.trower[1]);
                    monkey_list[l].inspected += 1;
                    monkey_list[l].items.remove(0); 
                }
           }

           println!("Monkey: {}: {:?}", m.id, m.items);
        }
    }
    let mut inspected = monkey_list.iter().map(|m| m.inspected).collect::<Vec<u64>>();
    inspected.sort();
    println!("Inspected: {:?}", inspected[inspected.len() - 1] * inspected[inspected.len() - 2]);
}


fn part_1(file: &str) { 
    let monkeys = file.trim().split("\n\n").collect::<Vec<&str>>();
    let mut monkey_list = parser(monkeys);
    for i in 0..20 {
        println!("After round {}, the monkeys are holding items with these worry levels:", i);
        for (l, m) in monkey_list.clone().iter().enumerate() {
            let monkey_listed = monkey_list.clone();
            for (j, item) in monkey_listed[l].items.iter().enumerate() {
                let mut new = 0;
                if m.old == 0 {
                    new = m.operation.make_operation(*item, *item) / 3;
                }
                else {
                    new = m.operation.make_operation(*item, m.old) / 3;
                }
                // println!("Operation: {:?}, new: {}, old: {}", m.operation, new, m.old);
                if new % m.test == 0 {
                    monkey_list[m.trower[0] as usize].items.push(new);
                    //println!("Item with {} of worry, because: {} from monkey {} trowed to {} monkey", new, item, m.id, m.trower[0]);
                    monkey_list[l].inspected += 1;
                    monkey_list[l].items.remove(0);  
                } else { 
                    monkey_list[m.trower[1] as usize].items.push(new);
                    // println!("Item with {} of worry, because: {} from monkey {} trowed to {} monkey", new, item, m.id, m.trower[1]);
                    monkey_list[l].inspected += 1;
                    monkey_list[l].items.remove(0); 
                }
           }

           println!("Monkey: {}: {:?}", m.id, m.items);
        }
    }
    let mut inspected = monkey_list.iter().map(|m| m.inspected).collect::<Vec<u64>>();
    inspected.sort();
    println!("Inspected: {:?}", inspected[inspected.len() - 1] * inspected[inspected.len() - 2]);
}

fn main() { 
    let file = std::fs::read_to_string("./input")
               .expect("Couldn't read the monkey's info");
    
    part_2(&file);    
}
