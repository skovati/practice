use std::fs;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn max_coord(&self) -> i32 {
        if self.x > self.y { self.x } else { self.y } }
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

    // loop over input and find largest point to make coordinate matrix
    let max_coord = lines
        .iter()
        .flatten()
        .max()
        .unwrap()
        .max_coord() as usize + 1;

    // make graph matrix to hold all lines
    let mut graph: Vec<Vec<i32>> = (0..max_coord).map(|_| vec![0i32; max_coord]).collect();

    // loop over point vectors and add to coordinate matrix
    for line in &lines {
        // if only y changes
        if line[0].x == line[1].x {
            for i in line[0].y..line[1].y {
                graph[(line[0].x as usize)][i as usize] += 1;
            }
        } 
        // otherwise, we know x changes
        else {
            for i in line[0].x..line[1].x {
                graph[(line[0].y as usize)][i as usize] += 1;
            }
        }
    }

    println!("{:?}", graph);

    // loop over updated graph and see how many are > 1
    let mut counter = 0;
    for x in &graph {
        for y in x {
            if *y > 1 {
                counter += 1;
            }
        }
    }

    println!("overlaps: {}", counter);
}
