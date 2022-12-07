fn get_unique_count(line: &String, count: usize) -> usize {
    for i in 0..line.len() - count {
        let s = String::from(&line[i..i+count]);
        let mut bytes = s.into_bytes();
        bytes.sort();
        bytes.dedup();

        if bytes.len() == count {
            return i + count;
        }
    }

    return 0;
}

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = aoc::read_lines("resources/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for rline in lines {
            if let Ok(line) = rline {
                println!("day1, part1: {}", get_unique_count(&line, 4));
                println!("day1, part2: {}", get_unique_count(&line, 14));
            }
        }
    }
}