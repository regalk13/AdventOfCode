// Cpu 
struct CPU {
    x: i32,
    cycle: i32,
}

impl CPU {
    fn add_cycle(&mut self, value: i32) { 
        self.x += value; 
    }
}


fn first_part(file: String) -> i32 {
    let mut cpu = CPU {
        x: 1,
        cycle: 0,
    };

    let mut signal_strength = vec![];
    for f in file.trim().split("\n") {
        let instruction  = f.split_once(" ");
        cpu.cycle += 1;
        if cpu.cycle == 20 || cpu.cycle == 60 || cpu.cycle == 100 || cpu.cycle == 140 || cpu.cycle == 180 || cpu.cycle == 220{
            println!("Signal: {}, cpu cycle: {}", cpu.x * cpu.cycle, cpu.cycle);
            signal_strength.push(cpu.x*cpu.cycle);
        }
        
        if instruction != None {
            let (_, value) = instruction.unwrap();
            cpu.cycle += 1;
            if cpu.cycle == 20 || cpu.cycle == 60 || cpu.cycle == 100 || cpu.cycle == 140 || cpu.cycle == 180 || cpu.cycle == 220{
                println!("Signal: {}, cpu cycle: {}", cpu.x * cpu.cycle, cpu.cycle); 
                signal_strength.push(cpu.x*cpu.cycle);
            }
            cpu.add_cycle(value.parse::<i32>().unwrap()); 
        }
        
    };

    println!("Cpu X: {}, cpu cycles: {}", cpu.x, cpu.cycle);
    signal_strength.iter().sum::<i32>() 
}

fn main() {
    let file = std::fs::read_to_string("./input").expect("Couldn't read input file");
    println!("Sum: {}",first_part(file));
}
