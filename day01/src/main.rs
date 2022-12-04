fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = aoc::read_lines("resources/input.txt") {
        let mut vec = Vec::new();
        vec.push(0);

        // Consumes the iterator, returns an (Optional) String
        for rline in lines {
            if let Ok(line) = rline {
                if line.is_empty() {
                    vec.push(0);
                    continue;
                }
                
                *vec.last_mut().unwrap() += line.parse::<i32>().unwrap();
            }
        }

        vec.sort_by(|a, b| b.cmp(a));
        println!("day1, part1: {}", vec[0]);
        println!("day1, part2: {}", vec[0] + vec[1] + vec[2]);
    }
}
