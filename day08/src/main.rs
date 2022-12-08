fn main() {
    let mut strings: Vec<String> = Vec::new();

    if let Ok(lines) = aoc::read_lines("resources/input.txt") {
        for rline in lines {
            if let Ok(line) = rline {
                strings.push(line);
            }
        }
    }

    let mut invisible: usize = 0;
    let len = strings[0].len();
    for row in 1..strings.len() - 1 {
        for col in 1..len -1 {
            let c = strings[row].chars().nth(col).unwrap();
            let max_left = &strings[row][0..col].chars().max().unwrap();
            if c > *max_left {
                continue;
            }

            let max_right = &strings[row][col+1..].chars().max().unwrap();
            if c > *max_right {
                continue;
            }

            let mut column_string = String::new();
            for s in &strings {
                column_string.push(s.chars().nth(col).unwrap());
            }

            let max_top = &column_string[0..row].chars().max().unwrap();
            if c > *max_top {
                continue;
            }

            let max_down = &column_string[row+1..].chars().max().unwrap();
            if c > *max_down {
                continue;
            }
            invisible += 1;
        }
    }

    println!("day8, part1: {}", strings.len() * len - invisible);

    let mut highest_scenic_score = 0;
    for row in 1..strings.len() - 1 {
        for col in 1..len -1 {
            let c = strings[row].chars().nth(col).unwrap();
            let mut l = col;
            let mut r = len - col - 1;
            let mut t = row;
            let mut d = strings.len() - row - 1;

            for i in (0..col).rev() {
                if strings[row].chars().nth(i).unwrap() >= c {
                    l = col - i;
                    break;
                }
            }

            for i in col+1..len {
                if strings[row].chars().nth(i).unwrap() >= c {
                    r = i - col;
                    break;
                }
            }

            for i in (0..row).rev() {
                if strings[i].chars().nth(col).unwrap() >= c {
                    t = row - i;
                    break;
                }
            }

            for i in row+1..strings.len() {
                if strings[i].chars().nth(col).unwrap() >= c {
                    d = i - row;
                    break;
                }
            }

            let scenic = r * l * t * d;
            if scenic > highest_scenic_score {
                highest_scenic_score = scenic;
            }
        }
    }

    println!("day8, part2: {}", highest_scenic_score);
}
