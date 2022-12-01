use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("resources/input.txt") {
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
        println!("highest calory: {}", vec[0]);
        println!("top-3 calories: {}", vec[0] + vec[1] + vec[2]);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
