
fn get_char_value(c: char) -> u32 {
    if c >= 'a' {
        return (c as u32) - 96;
    } else {
        return (c as u32) - 38;
    }
}

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = aoc::read_lines("resources/input.txt") {

        // part 1 variables
        let mut sum: u32 = 0;

        // part 2 variables
        let mut index: usize = 0;
        let mut sum2: u32 = 0;
        let mut store3lines : [Vec<char>; 3] = core::array::from_fn(|_| Vec::new());
        'outer: for rline in lines {

            if let Ok(line) = rline {
                store3lines[index] = line.chars().collect::<Vec<_>>();

                index += 1;
                // We have the latest 3 lines, lets find common element.
                if index == 3 {
                    'inner: for i in 0..store3lines[0].len() {
                        for j in 0..store3lines[1].len() {
                            for k in 0..store3lines[2].len() {
                                if store3lines[0][i] == store3lines[1][j] && store3lines[1][j] == store3lines[2][k] {
                                    sum2 += get_char_value(store3lines[0][i]);
                                    break 'inner;
                                }
                            }
                        }
                    }
                    index = 0;
                }

                for i in 0..line.len() / 2 {
                    for j in line.len() / 2..line.len() {
                        let lchar = line.chars().nth(i).unwrap();
                        let rchar = line.chars().nth(j).unwrap();
                        if lchar == rchar {
                            sum += get_char_value(lchar);
                            continue 'outer;
                        }
                    }
                }
            }
        }

        println!("day3, part1: {}", sum);
        println!("day3, part2: {}", sum2);
    }
}
