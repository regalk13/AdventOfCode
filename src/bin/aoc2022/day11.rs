// Operation enum, with the two options aviable
#[derive(Debug, Default, Clone)]
enum Operation {
    #[default]
    Sum,
    Multiply,
}

// Implement the operations
impl Operation {
    // Just return the operation for the first part
    fn make_operation(&self, old: u64, op: u64) -> u64 {
        // Matching the option
        match self {
            Operation::Sum => old + op,
            Operation::Multiply => old * op,
        }
    }
    // Return the mod magic_trick of the operation
    // See the definition down of the magic_trick
    fn make_operation_part2(&self, old: u64, op: u64, magic_trick: u64) -> u64 {
        match self {
            Operation::Sum => (old + op) % magic_trick,
            Operation::Multiply => (old * op) % magic_trick,
        }
    }
}

// Struct the monkey / u64 cause second part
#[derive(Debug, Default, Clone)]
struct Monkey {
    // Id of the monkey
    id: i32,
    // Items the monkey have, monkey return me this!! >:(
    items: Vec<u64>,
    // The operation the monkey make
    operation: Operation,
    // Value used for the operation new = old + op
    old: u64,
    // Test value used for the mod, and the magic trick!
    test: u64,
    // Monkey to trow, trow[0] monkey if the condition is true, else trow[1] monkey false!
    trower: Vec<u64>,
    // Number of items inspected
    inspected: u64,
}

// Parsing the input, a lot of work!
fn parser(monkeys: Vec<&str>) -> Vec<Monkey> {
    // Defining the list of monkeys
    let mut monkey_list: Vec<Monkey> = vec![];
    // Looping into each line
    for monkey in monkeys {
        // Creating a monkey, new monkey here!
        let mut monkey_struct = Monkey::default();
        // Looping inside each monkey
        for m in monkey.split("\n").collect::<Vec<&str>>() {
            // Add the monkey id, Monkey: id;
            if m.starts_with("Monkey") {
                let (_, id) = m.split_once(" ").unwrap();
                monkey_struct.id = id.trim_end_matches(":").parse::<i32>().unwrap();
            // Add monkey initial items
            } else if m.trim().starts_with("S") {
                let (_, items) = m.split_once(":").unwrap();
                // Parsing to u64
                let items = items
                    .trim()
                    .split(", ")
                    .map(|item| item.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();

                monkey_struct.items = items;
            // Adding operation
            } else if m.trim().starts_with("Op") {
                let (_, operation) = m.split_once("= ").unwrap();
                // Checking the operation and add it!
                if operation.contains("*") {
                    let (_, number) = operation.split_once("* ").unwrap();
                    // Check if we need to multiply the number of itself
                    if number == "old" {
                        // for later ;)
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
            // Adding number to test with the mod
            // This give us the monkey to trow the item
            } else if m.trim().starts_with("Te") {
                let (_, test) = m.split_once(":").unwrap();
                let (_, test) = test.split_once("by ").unwrap();
                monkey_struct.test = test.parse::<u64>().unwrap();
            // Get the two monkeys monkeys
            } else if m.trim().starts_with("If") {
                let (_, trow) = m.split_once("monkey ").unwrap();
                monkey_struct.trower.push(trow.parse::<u64>().unwrap());
            }
        }
        // Adding the little monkey to monkey friends list
        monkey_list.push(monkey_struct);
    }
    // Return the monkey list!
    monkey_list
}

// Part 2 function
fn part_2(file: &str) {
    // Split the file input by lines
    let monkeys = file.trim().split("\n\n").collect::<Vec<&str>>();
    // Get the monkey list parsed
    let mut monkey_list = parser(monkeys);
    // The magic trick, this is the number that controls the operation
    // magick tric = the multiplication of all the test numbers (they are prime numbers)
    // check the reddit for more information about this
    let magic_trick = monkey_list
        .iter()
        .map(|monkey| monkey.test)
        .product::<u64>();
    // 10_000 rounds!
    for i in 0..10_000 {
        println!(
            "After round {}, the monkeys are holding items with these worry levels:",
            i
        );
        // Cycle of each monkey
        for (l, m) in monkey_list.clone().iter().enumerate() {
            let monkey_listed = monkey_list.clone();
            // Cycle for each item the monkey currently have
            for item in &monkey_listed[l].items {
                let mut new = 0;

                if m.old == 0 {
                    new = m.operation.make_operation_part2(*item, *item, magic_trick);
                } else {
                    new = m.operation.make_operation_part2(*item, m.old, magic_trick);
                }
                // Check the new, that is the mod of the operation with the magic trick
                if new % m.test == 0 {
                    monkey_list[m.trower[0] as usize].items.push(new);
                    // Uncomment for debug
                    //println!("Item with {} of worry, because: {} from monkey {} trowed to {} monkey", new, item, m.id, m.trower[0]);
                    monkey_list[l].inspected += 1;
                    monkey_list[l].items.remove(0);
                } else {
                    monkey_list[m.trower[1] as usize].items.push(new);
                    // Uncomment for debug
                    // println!("Item with {} of worry, because: {} from monkey {} trowed to {} monkey", new, item, m.id, m.trower[1]);
                    monkey_list[l].inspected += 1;
                    monkey_list[l].items.remove(0);
                }
            }
            // Current state of each monkey
            println!("Monkey: {}: {:?}", m.id, m.items);
        }
    }

    // Get the number of inspected items by monkey then sort it
    let mut inspected = monkey_list
        .iter()
        .map(|m| m.inspected)
        .collect::<Vec<u64>>();
    inspected.sort();
    // The result is the product of the two last items in the sorted list
    println!(
        "Result: {:?}",
        inspected[inspected.len() - 1] * inspected[inspected.len() - 2]
    );
}

// Part 1 function
fn part_1(file: &str) {
    // Split the file input by lines
    let monkeys = file.trim().split("\n\n").collect::<Vec<&str>>();
    // Get the monkey list parsed
    let mut monkey_list = parser(monkeys);
    // 20 rounds of monkeys trowing your items between they
    for i in 0..20 {
        println!(
            "After round {}, the monkeys are holding items with these worry levels:",
            i
        );
        // Get the monkey and id
        for (l, m) in monkey_list.clone().iter().enumerate() {
            // Again clone the list for get the last items
            let monkey_listed = monkey_list.clone();
            // Cyclying for each item the monkey have
            for item in &monkey_listed[l].items {
                let mut new = 0;
                // Multiply by itself
                if m.old == 0 {
                    new = m.operation.make_operation(*item, *item) / 3;
                }
                // Just make the operation
                else {
                    new = m.operation.make_operation(*item, m.old) / 3;
                }
                // println!("Operation: {:?}, new: {}, old: {}", m.operation, new, m.old);
                // check the test if it's true throw the item to the monkey in index 0
                if new % m.test == 0 {
                    monkey_list[m.trower[0] as usize].items.push(new);
                    //println!("Item with {} of worry, because: {} from monkey {} trowed to {} monkey", new, item, m.id, m.trower[0]);
                    monkey_list[l].inspected += 1;
                    monkey_list[l].items.remove(0);
                // throw the item to the monkey in index 1
                } else {
                    monkey_list[m.trower[1] as usize].items.push(new);
                    // println!("Item with {} of worry, because: {} from monkey {} trowed to {} monkey", new, item, m.id, m.trower[1]);
                    monkey_list[l].inspected += 1;
                    monkey_list[l].items.remove(0);
                }
            }
            // State in this round of the monkey
            println!("Monkey: {}: {:?}", m.id, m.items);
        }
    }
    // Get the number of inspected items by monkey then sort it
    let mut inspected = monkey_list
        .iter()
        .map(|m| m.inspected)
        .collect::<Vec<u64>>();
    inspected.sort();
    // The result is the product of the two last items in the sorted list
    println!(
        "Result: {:?}",
        inspected[inspected.len() - 1] * inspected[inspected.len() - 2]
    );
}

fn main() {
    let file = std::fs::read_to_string("./input").expect("Couldn't read the monkey's info");

    part_1(&file);
    // part_2(&file);
}
