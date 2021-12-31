use std::fs;

fn main() {
    const MASKS: [i32; 12] = [
        0b1000_0000_0000,
        0b0100_0000_0000,
        0b0010_0000_0000,
        0b0001_0000_0000,
        0b0000_1000_0000,
        0b0000_0100_0000,
        0b0000_0010_0000,
        0b0000_0001_0000,
        0b0000_0000_1000,
        0b0000_0000_0100,
        0b0000_0000_0010,
        0b0000_0000_0001,
    ];

    let mut totals = [0; 12];

    // part 1
    if let Ok(input) = fs::read_to_string("input/day03.txt") {
        let entries = input.lines()
            .map(|l|
                i32::from_str_radix(l, 2).unwrap())
            .collect::<Vec<i32>>();
         
        for v in &entries {
            for bit in 0..12 {
                totals[bit] += (v & MASKS[bit]) / MASKS[bit];
            }
        }

        let size = entries.len() as i32;

        let gamma = totals.iter()
            .map(|b| b / (size/2))
            .zip(MASKS.iter())
            .fold(0, |a, (x, y)| a + (x * y));

        let epsilon = 2_i32.pow(12) - 1 - gamma;

        println!("gamma: {}, epsilon: {}, product: {}", gamma, epsilon, gamma * epsilon);
    }
}
