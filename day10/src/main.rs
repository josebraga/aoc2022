struct CPU {
    tick: usize,
    reg: i32,
    busy: usize,
    queue: i32,
}

impl CPU {
    fn noop(&mut self) {
        self.busy += 1;
    }

    fn addx(&mut self, amount: i32) {
        self.busy += 2;
        self.queue = amount;
    }

    fn is_free(&self) -> bool {
        return self.busy == 0;
    }

    fn run(&mut self) -> usize {
        if self.busy > 0 {
            self.busy -= 1;
            if self.busy == 0 {
                self.reg += self.queue;
                self.queue = 0;
            }
        }

        return self.tick;
    }

    fn advance(&mut self) {
        self.tick += 1;
    }

    fn reg(&self) -> i32 {
        return self.reg;
    }

    fn new() -> CPU {
        CPU {
            tick: 1,
            reg: 1,
            busy: 0,
            queue: 0,
        }
    }
}

fn is_tick_relevant(cpu: &CPU, tick: usize) -> i32 {
    if tick == 20 {
        return tick as i32 * cpu.reg();
    }

    for i in 1..20 {
        if tick == 20 + i * 40 {
            return tick as i32 * cpu.reg();
        }
    }

    return 0;
}

fn main() {
    let mut vec: Vec<String> = Vec::new();

    if let Ok(lines) = aoc::read_lines("resources/input.txt") {
        for rline in lines {
            if let Ok(line) = rline {
                vec.push(line);
            }
        }
    }

    let mut cpu = CPU::new();
    let mut total_p1: i32 = 0;

    let mut screen = (0..240).map(|_| " ").collect::<String>();

    loop {
        // Advance a step and check if we can get value from CPU.
        let tick = cpu.run();
        let sprite_pos = cpu.reg();

        if (tick % 40) as i32 >= sprite_pos && (tick % 40) as i32 <= sprite_pos + 2 {
            screen.replace_range(tick - 1..tick, "#");
        }

        total_p1 += is_tick_relevant(&cpu, tick);

        if cpu.is_free() {
            let line = vec.remove(0);
            // Handle commands.
            if line == "noop" {
                cpu.noop();
            } else if &line[0..4] == "addx" {
                let amount = line.split_whitespace().nth_back(0).unwrap_or("");
                cpu.addx(amount.parse::<i32>().unwrap());
            }
        }

        cpu.advance();
        if vec.is_empty() {
            break;
        }
    }

    println!("day10, part1: {}", total_p1);

    // Start of part 2
    println!("day10, part2:");
    for i in 0..6 {
        println!("{}", &screen[i*40..(i+1)*40 - 1]);
    }
}
