fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = aoc::read_lines("resources/input.txt") {
        let mut vec = Vec::new();
        vec.push(0);

        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if ip.is_empty() {
                    vec.push(0);
                    continue;
                }
                
                *vec.last_mut().unwrap() += ip.parse::<i32>().unwrap();
            }
        }

        vec.sort_by(|a, b| b.cmp(a));
        println!("day1, part1 - highest calory: {}", vec[0]);
        println!("day1, part2 - top-3 calories: {}", vec[0] + vec[1] + vec[2]);
    }
}
