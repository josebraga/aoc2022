const P1_SIZE: u32 = 100000;
const P2_MIN_SIZE: u32 = 30000000;
const P2_FS_SIZE: u32 = 70000000;

fn main() {
    let mut stack: Vec<u32> = Vec::new();
    let mut folders_size: Vec<u32> = Vec::new();

    let mut sum_folders_p1: u32 = 0;

    if let Ok(lines) = aoc::read_lines("resources/input.txt") {
        for rline in lines {
            if let Ok(line) = rline {
                // ignore `ls` and `dir`
                if &line[0..3] == "dir" ||  line == "$ ls" {
                    continue
                }

                // now we either have a `cd` command, or a file size.
                if line.chars().nth(0).unwrap() == '$' {
                    let target = &line[5..];

                    if target == ".." {
                        let pathsize = stack.last().unwrap().clone();

                        folders_size.push(pathsize);

                        if pathsize < P1_SIZE {
                            sum_folders_p1 += pathsize;
                        }
                        
                        let size = stack.pop().unwrap();
                        let depth = stack.len();
                        stack[depth - 1] += size;
                    } else {
                        stack.push(0);
                    }
                } else {
                    // all it's left now are file sizes
                    let filesize = line.split_whitespace()
                        .next()
                        .unwrap_or("");
                    
                    let depth = stack.len();
                    stack[depth - 1] += filesize.parse::<u32>().unwrap();
                }
            }
        }
    }

    println!("day7, part1: {}", sum_folders_p1);

    let filesystem_size = stack.iter().sum::<u32>();
    let must_freeup_size = P2_MIN_SIZE - (P2_FS_SIZE-filesystem_size);

    let mut del_folder_size: u32 = filesystem_size;
    for size in &folders_size {
        if size > &must_freeup_size && size < &del_folder_size {
            del_folder_size = *size;
        }
    }

    println!("day7, part2: {}", del_folder_size);
}
