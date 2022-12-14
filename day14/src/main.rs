// Sufficient for part2's "infinite" map
const BIG_MAP: usize = 1400;

#[derive(Debug)]
struct Line {
    points: Vec<(usize, usize)>,
}

fn parse(s: &str) -> (usize, usize) {
    let mut split = s.split(",");
    let s_x = split.next().unwrap();
    let s_y = split.next().unwrap();
    let x = s_x.parse::<usize>().unwrap();
    let y = s_y.parse::<usize>().unwrap();
    return (x, y);
}

fn build_grid(structure: &mut Vec<Line>, limits: (usize, usize, usize, usize), p2: bool) -> Vec<String> {
    let cell_size = (limits.1 - limits.0 + 1, limits.3 - limits.2 + 1);
    let mut grid: Vec<String> = Vec::new();

    if p2 {
        for _ in 0..cell_size.1 + 1 {
            grid.push((0..BIG_MAP).map(|_| ".").collect::<String>());
        }
    
        grid.push((0..BIG_MAP).map(|_| "#").collect::<String>());
    } else {
        for _ in 0..cell_size.1 {
            grid.push((0..cell_size.0).map(|_| ".").collect::<String>());
        }
    }

    for block in structure.iter() {
        for point in 0..block.points.len() - 1 {
            let mut p0_x = if p2 { block.points[point].0 + BIG_MAP / 2 } else { block.points[point].0 - limits.0 };
            let mut p1_x = if p2 { block.points[point + 1].0 + BIG_MAP / 2 } else { block.points[point + 1].0 - limits.0 };
            let mut p0_y = block.points[point].1;
            let mut p1_y = block.points[point + 1].1;
            if p0_x == p1_x {
                if p0_y > p1_y {
                    std::mem::swap(&mut p0_y, &mut p1_y);
                }

                for p in p0_y..p1_y + 1 {
                    grid[p].replace_range(p0_x..p0_x + 1, "#");
                }
            }

            if p0_y == p1_y {
                if p0_x > p1_x {
                    std::mem::swap(&mut p0_x, &mut p1_x);
                }

                for p in p0_x..p1_x + 1 {
                    grid[p0_y].replace_range(p..p + 1, "#");
                }
            }
        }
    }

    return grid;
}

fn calc(grid: &mut Vec<String>, limits: (usize, usize, usize, usize), p2: bool) -> usize {
    let mut count = 0;
    'outer: loop {
        let mut point = if p2 { (0usize, 500 + BIG_MAP / 2) } else { (0usize, 500 - limits.0) };

        'inner: loop {
            if point.0 == grid.len() - 1 {
                break 'outer;
            } else if grid[point.0 + 1].chars().nth(point.1).unwrap() == '.' {
                point.0 += 1;
                continue 'inner;
            } else if point.1 == 0 {
                break 'outer;
            } else if grid[point.0 + 1].chars().nth(point.1 - 1).unwrap() == '.' {
                point.0 += 1;
                point.1 -= 1;
                continue 'inner;
            } else if grid[point.0 + 1].chars().nth(point.1 + 1).unwrap() == '.' {
                point.0 += 1;
                point.1 += 1;
                continue 'inner;
            } else {
                grid[point.0].replace_range(point.1..point.1 + 1, "o");
                count += 1;
                
                if point.0 == 0 {
                    break 'outer;
                }

                continue 'outer;
            }
        }
    }

    count
}

fn paint_grid(grid: &Vec<String>) {
    // paint grid
    for line in grid.iter() {
        println!("{}", line);
    }
}

fn main() {
    // Read from file and build a structure, which is a vector with all line points.
    let mut structure: Vec<Line> = Vec::new();
    let mut limits = (0usize, 0usize, 0usize, 0usize);
    if let Ok(lines) = aoc::read_lines("resources/input.txt") {
        for rline in lines {
            if let Ok(line) = rline {
                let split = line.split(" -> ");
                let vec = split.collect::<Vec<&str>>();

                structure.push(Line { points: Vec::new() });
                for point in vec {
                    let coords = parse(point);
                    if let Some(last) = structure.last_mut() {
                        last.points.push(coords);
                    }

                    if limits == (0, 0, 0, 0) {
                        limits.0 = coords.0;
                        limits.1 = coords.1;
                        limits.3 = coords.1;
                        continue;
                    }

                    if coords.0 < limits.0 {
                        limits.0 = coords.0;
                    }
                    if coords.0 > limits.1 {
                        limits.1 = coords.0;
                    }
                    if coords.1 > limits.3 {
                        limits.3 = coords.1;
                    }
                }
            }
        }
    }

    // Now let's translate vector with all line points into a grid made up of strings.
    let mut grid = build_grid(&mut structure, limits, false);

    // Paint grid before
    paint_grid(&grid);

    let mut count = calc(&mut grid, limits, false);

    // Paint grid after part 1
    paint_grid(&grid);

    println!("day14, part1: {}", count);

    // For part 2 we have to build a new grid, bigger, and recompute
    let mut grid = build_grid(&mut structure, limits, true);
    count = calc(&mut grid, limits, true);
    println!("day14, part2: {}", count);

}
