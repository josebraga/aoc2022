use serde_json::Value;

#[derive(PartialEq)]
enum IndexPos {
    Under2,
    Under6,
    Over,
}

#[derive(PartialEq)]
enum Order {
    LeftIsLower,
    RightIsLower,
    Equal,
}
fn compare_vec(v1: &mut Vec<Value>, v2: &mut Vec<Value>) -> Order {
    if v1.is_empty() && !v2.is_empty() {
        return Order::LeftIsLower;
    } else if !v1.is_empty() && v2.is_empty() {
        return Order::RightIsLower;
    }

    let size1 = v1.len();
    let size2 = v2.len();

    let sz = std::cmp::min(size1, size2);

    for _ in 0..sz {
        let ret = compare(v1.remove(0), v2.remove(0));
        if ret == Order::Equal {
            continue
        }

        return ret;
    }

    if size1 < size2 {
        return Order::LeftIsLower;
    } else if size1 > size2 {
        return Order::RightIsLower;
    }

    return Order::Equal;
}

fn compare(mut v1: Value, mut v2: Value) -> Order {
    if v1.is_array() && v2.is_array() {
        let mut v1arr = v1.as_array_mut().unwrap().clone();
        let mut v2arr = v2.as_array_mut().unwrap().clone();
        return compare_vec(&mut v1arr,&mut v2arr);
    }

    if v1.is_number() && v2.is_number() {
        if v1.as_u64() < v2.as_u64() {
            return Order::LeftIsLower;
        } else if v1.as_u64() > v2.as_u64() {
            return Order::RightIsLower;
        }
    }

    if v1.is_number() && v2.is_array() {
        let mut v1_promoted_to_vec = vec![v1];
        let mut v2arr = v2.as_array_mut().unwrap().clone();
        return compare_vec(&mut v1_promoted_to_vec,&mut v2arr);
    } else if v1.is_array() && v2.is_number() {
        let mut v1arr = v1.as_array_mut().unwrap().clone();
        let mut v2_promoted_to_vec = vec![v2];
        return compare_vec(&mut v1arr,&mut v2_promoted_to_vec);
    }

    return Order::Equal;
}


fn open_vec(v: &mut Vec<Value>) -> IndexPos {
    if v.is_empty() {
        return IndexPos::Under2;
    }

    return open(v.remove(0));
}
fn open(mut v: Value) -> IndexPos {
    if v.is_array() {
        let mut arr = v.as_array_mut().unwrap().clone();
        return open_vec(&mut arr);
    }

    if v.as_u64().unwrap() < 2 {
        return IndexPos::Under2;
    } else if v.as_u64().unwrap() >= 2 && v.as_u64().unwrap() < 6 {
        return IndexPos::Under6;
    } else {
        return IndexPos::Over;
    }
}

fn part1(s1: &String, s2: &String) -> Order {
    let mut outer1: Vec<Value> = serde_json::from_str(s1).unwrap();
    let mut outer2: Vec<Value> = serde_json::from_str(s2).unwrap();

    return compare_vec(&mut outer1, &mut outer2);
}

fn part2(s: &String) -> IndexPos {
    let mut outer: Vec<Value> = serde_json::from_str(s).unwrap();
    return open_vec(&mut outer);
}

fn main() {
    let mut first_string = String::from("");
    let mut first = true;
    let mut part1_acc = 0;
    let mut part2_index2 = 1;
    let mut part2_index6 = 2;
    let mut index = 1;
    if let Ok(lines) = aoc::read_lines("resources/input.txt") {
        for rline in lines {
            if let Ok(line) = rline {
                if line.is_empty() {
                    continue;
                }

                // part 2
                let p2 = part2(&line);
                if p2 == IndexPos::Under2 {
                    part2_index2 += 1;
                    part2_index6 += 1;
                } else if p2 == IndexPos::Under6 {
                    part2_index6 += 1;
                } 

                // part 1
                if first {
                    first_string = line;
                    first = false;
                } else {
                    first = true;
                    if part1(&first_string, &line) == Order::LeftIsLower {
                        part1_acc += index;
                    }
                    index += 1;
                }              
            }
        }
    }

    println!("day13, part1: {}", part1_acc);
    println!("day13, part2: {}", part2_index2 * part2_index6);
}
