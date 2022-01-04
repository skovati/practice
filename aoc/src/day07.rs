use std::fs;

/// Returns the minimimum amount of fuel needed to get all
/// input crabs to the same location horizontally
///
/// # Arguments
/// `pos` - A vector of i32s representing the initial horizontal
/// positions of crabs
fn horiz_fuel(pos: Vec<i32>) -> i32 {
    let max = pos.
        iter()
        .max()
        .unwrap()
        .clone();

    let min = pos.
        iter()
        .min()
        .unwrap()
        .clone();

    let mut fuels: Vec<i32> = Vec::new();
    for i in min..max {
        let cost = pos
            .iter()
            .map(|p| (p-i).abs())
            .sum();
        fuels.push(cost);
    }

    return fuels.iter().min().unwrap().clone();
}

fn main() {
    // parse input into vector
    let input: Vec<i32> = fs::read_to_string("input/day07.txt")
        .expect("file not found")
        .trim()
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    println!("part 1: {}", horiz_fuel(input));
}

/// tests the small input and answer given by aoc
#[test]
fn small_input() {
    assert_eq!(horiz_fuel(vec![16,1,2,0,4,2,7,1,2,14]), 37);
}
