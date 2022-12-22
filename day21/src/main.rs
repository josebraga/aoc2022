use std::collections::HashMap;

enum MathOperation {
    Plus,
    Minus,
    Multiply,
    Divide,
}

struct Operation {
    lhs: String,
    rhs: String,
    operation:  MathOperation,
}

impl Operation {
    fn solve(&self, monkeys: &HashMap<String, MonkeyJob>) -> i64 {
        let lhs = monkeys.get(&self.lhs).unwrap().solve(&monkeys);
        let rhs = monkeys.get(&self.rhs).unwrap().solve(&monkeys);
        return match self.operation {
            MathOperation::Plus => lhs + rhs,
            MathOperation::Minus => lhs - rhs,
            MathOperation::Multiply => lhs * rhs,
            MathOperation::Divide => lhs / rhs,
        };
    }
}

struct MonkeyJob {
    result: Option<i64>,
    operation: Option<Operation>,
}

impl MonkeyJob {
    fn solve(&self, monkeys: &HashMap<String, MonkeyJob>) -> i64 {
        match self.result {
            Some(r) => return r,
            None => {
                return self.operation.as_ref().unwrap().solve(&monkeys);
            }
        }
    }
}

fn main() {
    let mut monkeys: HashMap<String, MonkeyJob> = HashMap::new();
    if let Ok(lines) = aoc::read_lines("resources/input.txt") {
        for rline in lines {
            if let Ok(line) = rline {
                let tokens: Vec<&str> = line.split(" ").collect();
                let monkey_name = tokens[0][..4].to_string();
                if tokens.len() == 2 {
                    monkeys.insert(monkey_name, MonkeyJob {
                        result: Some(tokens[1].parse().unwrap()),
                        operation: None
                    });
                } else {
                    let op = Operation {
                        lhs: tokens[1].to_string(),
                        rhs: tokens[3].to_string(),
                        operation: match tokens[2] {
                            "+" => MathOperation::Plus,
                            "-" => MathOperation::Minus,
                            "*" => MathOperation::Multiply,
                            "/" => MathOperation::Divide,
                            &_ => todo!(),
                        }
                    };

                    monkeys.insert(monkey_name, MonkeyJob {
                        result: None,
                        operation: Some(op),
                    });
                }
            }
        }
    }

    println!("day21, part1: {}", monkeys["root"].solve(&monkeys));

    // Peeked `humn` original value, compute for a few different ranges, and
    // with some trial and error this was the winning range.
    for i in 3220993860000..3220993880000 {
        monkeys.insert("humn".to_string(), MonkeyJob {
            result: Some(i),
            operation: None
        });
    
        // monkeys used in `root` operation.
        if monkeys["zhfp"].solve(&monkeys) == monkeys["hghd"].solve(&monkeys) {
            println!("day21, part2: {}", i);
            break;
        }
    }
}
