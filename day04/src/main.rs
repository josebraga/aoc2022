
fn full_overlap(l1: u32, l2: u32, r1: u32, r2: u32) -> u32 {
    if (l1 >= r1 && l2 <= r2) || (r1 >= l1 && r2 <= l2) {
        return 1;
    } else {
        return 0;
    }
}

fn any_overlap(l1: u32, l2: u32, r1: u32, r2: u32) -> u32 {
    if (l1 >= r1 && l1 <= r2) || (l2 >= r1 && l2 <= r2) || 
        (r1 >= l1 && r1 <= l2) || (r2 >= l1 && r2 <= l2) {
        return 1;
    } else {
        return 0;
    }
}

fn main() {
    if let Ok(lines) = aoc::read_lines("resources/input.txt") {
        let mut sum: u32 = 0;
        let mut sum2: u32 = 0;
        for rline in lines {
            if let Ok(line) = rline {
                let sections: Vec<&str> = line.split(",").collect();
                let section_left: Vec<&str> = sections[0].split("-").collect();
                let section_right: Vec<&str> = sections[1].split("-").collect();

                let l1: u32 = section_left[0].parse().unwrap();
                let l2: u32 = section_left[1].parse().unwrap();
                let r1: u32 = section_right[0].parse().unwrap();
                let r2: u32 = section_right[1].parse().unwrap();

                sum += full_overlap(l1, l2, r1, r2);
                sum2 += any_overlap(l1, l2, r1, r2);
            }
        }

        println!("day4, part1: {}", sum);
        println!("day4, part2: {}", sum2);
    }
}
