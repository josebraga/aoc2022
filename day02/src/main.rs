use std::collections::HashMap;

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

fn fight(lhs: &str, rhs: &str) -> i32 {
    return match lhs {
        "A" => match rhs {
            "X" => 3,
            "Y" => 6,
            _ => 0,
        }

        "B" => match rhs {
            "X" => 0,
            "Y" => 3,
            _ => 6,
        }

        _ => match rhs {
            "X" => 6,
            "Y" => 0,
            _ => 3,
        }
    }
}

fn fix_strategy(lhs: &str, strategy: &str) -> String {
    return match lhs {
        "X" => match strategy {
            "A" => String::from("Z"),
            "B" => String::from("X"),
            _ => String::from("Y"),
        }

        "Y" => match strategy {
            "A" => String::from("X"),
            "B" => String::from("Y"),
            _ => String::from("Z"),
        }

        _ => match strategy {
            "A" => String::from("Y"),
            "B" => String::from("Z"),
            _ => String::from("X"),
        }
    }
}

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = aoc::read_lines("resources/input.txt") {

        let map_me: HashMap<&str, i32> = hashmap! { "X" => 1, "Y" => 2, "Z" => 3 };
        let mut sum_part1: i32 = 0;
        let mut sum_part2: i32 = 0;
        // Consumes the iterator, returns an (Optional) &str
        for rline in lines {
            if let Ok(line) = rline {
                let v: Vec<&str> = line.split(' ').collect();
                let s = &fix_strategy(v[0], v[1]);
                sum_part2 += map_me.get(s.as_str()).unwrap() + fight(v[0], s);
                sum_part1 += map_me.get(v[1]).unwrap() + fight(v[0], v[1]);

            }
        }

        println!("day2, part1: {}", sum_part1);
        println!("day2, part2: {}", sum_part2);
    }
}
