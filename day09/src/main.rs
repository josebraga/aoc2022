use std::cmp;

type Point = (i32, i32);
type ToMove = (i32, i32);

fn move_to(p: &mut Point, t: &mut ToMove) {
    if t.0 != 0 {
        if t.0 > 0 {
            t.0 -= 1;
            p.0 += 1;
        }

        if t.0 < 0 {
            t.0 += 1;
            p.0 -= 1;
        }
    } else if t.1 != 0 {
        if t.1 > 0 {
            t.1 -= 1;
            p.1 += 1;
        }

        if t.1 < 0 {
            t.1 += 1;
            p.1 -= 1;
        }
    }
}

fn approach(p: &Point, t: &mut ToMove) {
    let dist = cmp::max((p.0 - t.0).abs(), (p.1 - t.1).abs());
    if dist > 1 {
        if p.0 > t.0 {
            t.0 += 1;
        } else if p.0 < t.0 {
            t.0 -= 1;
        }

        if p.1 > t.1 {
            t.1 += 1;
        } else if p.1 < t.1 {
            t.1 -= 1;
        }
    }
}

fn parse(s: &String) -> ToMove {
    let distance = s.split_whitespace()
        .nth(1)
        .unwrap_or("");

    let value = distance.parse::<i32>().unwrap();

    match &s[0..1] {
        "R" => return (value, 0),
        "L" => return (-value, 0),
        "U" => return (0, value),
        "D" => return (0, -value),
        _ => return (0, 0),
    }
}

fn main() {
    let mut vec_tail: Vec<Point> = Vec::new();

    let mut head = (0, 0);
    let mut tail = (0, 0);
    
    // part 2 variables
    let mut array = [(0, 0); 10];
    let mut vec_longrope: Vec<Point> = Vec::new();

    if let Ok(lines) = aoc::read_lines("resources/input.txt") {
        for rline in lines {
            if let Ok(line) = rline {
                let mut tomove = parse(&line);
                let mut tomove_copy = tomove.clone();
                while tomove != (0, 0) {
                    move_to(&mut head, &mut tomove);
                    approach(&head, &mut tail);

                    if !vec_tail.contains(&tail) {
                        vec_tail.push(tail);
                    }

                    // part 2
                    move_to(&mut array[0], &mut tomove_copy);
                    for i in 0..array.len() - 1 {
                        let p = array[i].clone();
                        approach(&p, &mut array[i+1])
                    }
                    if !vec_longrope.contains(&array.last_mut().unwrap()) {
                        vec_longrope.push(array.last().unwrap().clone());
                    }

                }
            }
        }
    }

    println!("day9, part1: {}", vec_tail.len());
    println!("day9, part2: {}", vec_longrope.len());
}
