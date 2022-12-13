use std::cmp::Ordering;
use std::str::Chars;

#[derive(Debug)]
enum Val {
    Number(i32),
    List(Vec<Val>),
}

impl Val {
    fn parse(s: &str) -> Self {
        let mut c = s.chars();
        if c.next().unwrap() != '[' {
            panic!("Bad input");
        }
        Self::parse_into(&mut c)
    }
    fn parse_into(c: &mut Chars) -> Self {
        let mut result = Vec::new();
        let mut num = -1;
        while let Some(ch) = c.next() {
            match ch {
            '[' => result.push(Self::parse_into(c)),
            ',' => {
                    if num >= 0 {
                        result.push(Self::Number(num)); 
                        num = -1;
                    }
            }
            ']' => {
                if num >= 0 {
                    result.push(Self::Number(num));
                }
                return Self::List(result)},
            '0'..='9' => { 
                if num == -1 {
                    num = (ch as u8 - b'0') as i32;
                } else {
                    num = (num * 10) + (ch as u8 - b'0') as i32
                }
            },
            _ => panic!("Signal in bad state or smth '{ch}'"),
            }
            }
        Val::List(result)
    }
    fn compare(&self, other: &Self) -> Ordering {
        match(self, other) {
            (Val::List(left), Val::List(right)) => {
                let mut idx = 0;
                loop {
                if left.len() <= idx || right.len() <= idx {
                    if left.len() < right.len() {
                        return Ordering::Less;
                    } else if left.len() == right.len() {
                        return Ordering::Equal;
                    } else {
                        return Ordering::Greater;
                    }
                }
                match (&left[idx], &right[idx]) {
                    (Val::Number(l), Val::Number(r)) => 
                    {
                        if l < r { 
                            return Ordering::Less; 
                        } else if l > r {
                            return Ordering::Greater;
                        }
                    }
                    (Val::List(_), Val::Number(r)) => {
                        let check = left[idx].compare(&Val::List(vec![Val::Number(*r)]));
                        if check != Ordering::Equal {
                            return check;
                        }
                    }
                    (Val::Number(l), Val::List(_)) => {
                        let check = Val::List(vec![Val::Number(*l)]).compare(&right[idx]);
                        if check != Ordering::Equal {
                            return check;
                        }
                    }
                    (Val::List(_), Val::List(_)) => {
                        let check = left[idx].compare(&right[idx]);
                        if check != Ordering::Equal {
                            return check;
                        }
                    }
                }
                idx += 1;
            }
        }
         _ => panic!("Invalid input")        
    }
}
}


fn first_part(file: &str) {
    let signals = file.split("\n").filter(|f| *f != "").collect::<Vec<&str>>();
    let mut output = 0;
    let mut pairs: Vec<(Val, Val)> = vec![];
    for pair in signals.chunks(2) {
        let left = Val::parse(&pair[0]);
        let right = Val::parse(&pair[1]);
        pairs.push((left,right));
    }

    for (i, p) in pairs.iter().enumerate() {
        if p.0.compare(&p.1) == Ordering::Less {
            output += i + 1;
        }
    }
    println!("Output: {}", output);
}

fn main() {
    let file = std::fs::read_to_string("./input").expect("Can't read the signal input");
    
    

    first_part(&file);
}
