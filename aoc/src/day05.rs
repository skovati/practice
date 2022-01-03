use std::fs;
use itertools::Itertools;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // parse input into starting Vec<Point> and ending Vec<Point>
    let input = fs::read_to_string("input/day05.txt")
        .expect("File not found!");
    let lines = input.lines()
        // each line
        .map(|line| {
            line.split(" -> ")
                // each point
                .map(|point| {
                    point
                        // get each x or y val as i32's
                        .split(',')
                        .map(|n| n.parse::<i32>().unwrap())
                        .tuples()
                        // and map to struct
                        .map(|(x, y)| Point {
                            x, 
                            y,
                        })
                        .next()
                        .unwrap()
                })
                .collect::<Vec<Point>>()
        })
        .collect::<Vec<Vec<Point>>>();
    }

    // loop over input and find largest point to make coordinate matrix
    let max = lines
        .iter()
        .flatten();

    println!("max: {:?}", max);

    // loop over point vectors and add to coordinate matrix
    
    // 

}
