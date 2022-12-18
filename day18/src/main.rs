use std::collections::HashSet;

fn main() {
    let mut max = (0, 0, 0);
    let mut points: Vec<(i32, i32, i32)> = Vec::new();
    let mut neighbors: HashSet<(usize, usize)> = HashSet::new();

    if let Ok(lines) = aoc::read_lines("resources/input.txt") {
        for rline in lines {
            if let Ok(line) = rline {
                let coords: Vec<&str> = line.split(",").collect();
                let p = (coords[0].parse::<i32>().unwrap(), 
                    coords[1].parse::<i32>().unwrap(),
                    coords[2].parse::<i32>().unwrap());

                points.push(p);

                if p.0 > max.0 {
                    max.0 = p.0;
                }
                if p.1 > max.1 {
                    max.1 = p.1;
                }
                if p.2 > max.2 {
                    max.2 = p.2;
                }
            }
        }
    }

    // Part 1, for every point, check if it is next to another point.
    for i in 0..points.len() {
        for j in 0..points.len() {
            if i == j {
                continue;
            }

            for x in 0..3 {
                for delta in [-1, 1] {
                    let mut offset: [i32; 3] = [0, 0, 0];
    
                    offset[x] = delta;
                    let test = (points[j].0 + offset[0], points[j].1 + offset[1], points[j].2 + offset[2]);

                    if points[i] == test {
                        if i < j {
                            neighbors.insert((i, j));
                        } else {
                            neighbors.insert((j, i));
                        }
                    }
                }
            }
        }
    }

    println!("day18, part1: {}", points.len() * 6 - neighbors.len() * 2);

    // Part 2, start from the outside and visit all possible cubes. When a
    // neighbor is found that belongs to the droplet, increment count.
    let mut surfaces = 0;
    let mut to_visit: Vec<(i32, i32, i32)> = Vec::new();
    let mut visited: HashSet<(i32, i32, i32)> = HashSet::new();

    to_visit.push((0, 0, 0));

    while to_visit.len() > 0 {
        let p = to_visit[0];
        
        if visited.contains(&p) {
            to_visit.remove(0);
            continue;
        }

        visited.insert(p);

        let mut neighbors: Vec<(i32, i32, i32)> = Vec::new();

        for i in 0..3 {
            for delta in [-1, 1] {
                let mut offset: [i32; 3] = [0, 0, 0];

                offset[i] = delta;
                let test = (p.0 + offset[0], p.1 + offset[1], p.2 + offset[2]);

                if test.0 > max.0 + 1 || test.1 > max.1 + 1 || test.2 > max.2 + 1 ||
                    test.0 < -1 || test.1 < -1 || test.2 < -1 {
                    continue
                }

                // Found a surface, otherwise found a place to visit.
                if points.contains(&test) {
                    surfaces += 1;
                } else {
                    neighbors.push(test);
                }
            }
        }

        to_visit.remove(0);
        for n in neighbors.iter() {
            to_visit.push(n.clone());
        }
    }

    println!("day18, part2: {}", surfaces);
}
