use std::str;
use std::fs;

fn main() {
    if let Ok(input) = fs::read_to_string("input/day02.txt") {
        let mut horiz = 0;
        let mut depth = 0;
        let vec_per_line = input.lines()
            .map(|l|
                l.split_whitespace()
                    .collect::<Vec<&str>>());
        vec_per_line.for_each(|c|
            match c[0] {
                "forward" => {
                    horiz += c[1].parse::<i32>().unwrap();
                },
                "down" => {
                    depth += c[1].parse::<i32>().unwrap();
                },
                "up" => {
                    depth -= c[1].parse::<i32>().unwrap();
                }
                _ => {}
            });
        println!("horiz: {}, depth: {}, product: {}", horiz, depth, horiz*depth);
    }
}

