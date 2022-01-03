use std::fs;
use std::cmp;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn max_coord(&self) -> usize {
        if self.x > self.y { self.x } else { self.y } }
}

fn print_graph(graph: &Vec<Vec<usize>>) {
    println!("[");
    for col in 0..graph.len() {
        print!("[");
        for row in 0..graph[col].len() {
            print!("{}", graph[row][col]);
        }
        println!("]");
    }
    print!("]");
    println!();
}

fn count_overlaps(graph: &Vec<Vec<usize>>) -> usize {
    graph
        .iter()
        .flatten()
        .filter(|p| **p > 1)
        .count()
}

fn populate(graph: &mut Vec<Vec<usize>>, lines: &Vec<Vec<Point>>) {
    for line in lines {
        let min_x = cmp::min(line[0].x, line[1].x);
        let min_y = cmp::min(line[0].y, line[1].y);
        let max_x = cmp::max(line[0].x, line[1].x);
        let max_y = cmp::max(line[0].y, line[1].y);
        let x_dec = line[0].x > line[1].x;
        let y_dec = line[0].y > line[1].y;

        // if we have a vertical line
        if min_x == max_x {
            for y in min_y..max_y+1 {
                graph[min_x][y] += 1;
            }
        } 
        // or if we have a horizontal line
        else if min_y == max_y {
            for x in min_x..max_x+1 {
                graph[x][min_y] += 1;
            }
        }
        // otherwise, we have a diagonal
        // and we know diff(min_y, max_y) == diff(min_x, max_x)
        // so we can safely iterate and increment both evenly
        else {
            // if x is decreasing but y is increasing, we have a negative slope
            // so we need to reverse the x iterator 
            if x_dec && !y_dec {
                let iter = ((min_x..max_x+1).rev()).zip(min_y..max_y+1);
                for (x, y) in iter {
                    graph[x][y] += 1;
                }
            }
            // same if y is dec but x is inc
            else if !x_dec && y_dec {
                let iter = (min_x..max_x+1).zip((min_y..max_y+1).rev());
                for (x, y) in iter {
                    graph[x][y] += 1;
                }
            }
            // otherwise, we have positive slope, and dont have to worry about it
            else {
                let iter = (min_x..max_x+1).zip(min_y..max_y+1);
                for (x, y) in iter {
                    graph[x][y] += 1;
                }
            }
        }
    }
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
                        // get each x or y val as usize's
                        .split(',')
                        .map(|n| n.parse::<usize>().unwrap())
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
    let max = lines
        .iter()
        .flatten()
        .max()
        .unwrap()
        .max_coord() as usize + 1;

    let axial_lines: Vec<Vec<Point>> = lines
        .iter()
        .cloned()
        .filter(|point| point[0].x == point[1].x || point[0].y == point[1].y)
        .collect();

    // make graph matrix to hold all lines
    let mut graph: Vec<Vec<usize>> = (0..max)
        .map(|_| vec![0usize; max])
        .collect();

    // part 1
    // loop over point vectors and add to coordinate matrix
    populate(&mut graph, &axial_lines);

    // loop over updated graph and see how many are > 1

    println!("axial overlaps: {}", count_overlaps(&graph));

    let diag_lines: Vec<Vec<Point>> = lines
        .iter()
        .cloned()
        .filter(|point| point[0].x != point[1].x && point[0].y != point[1].y)
        .collect();

    // part 2
    // loop over point vectors and add to coordinate matrix
    populate(&mut graph, &diag_lines);

    // loop over updated graph and see how many are > 1

    println!("diag overlaps: {}", count_overlaps(&graph));
}
