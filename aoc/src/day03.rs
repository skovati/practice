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

    // part 2
    if let Ok(input) = fs::read_to_string("input/day03.txt") {
        // parse into vector of i32s
        let entries = input.lines()
            .map(|l|
                i32::from_str_radix(l, 2).unwrap())
            .collect::<Vec<i32>>();

        let mut o2_rating = entries.clone();
        let mut size: i32;

        // loop over all 12 bit positions
        for bit in MASKS {
            size = o2_rating.len() as i32;
            // if 1 is the most common bit in this position, which is calculated by taking the
            // bitwise and with each bit mask, and dividing by said bitmask, which will result in
            // either 1 or 0, summing those mappings, and seeing how it compares to size/2
            if o2_rating.iter().map(|e| (e & bit) / bit).sum::<i32>() as f32 >= (size as f32 / 2.0) {
                // then filter to keep only entries that have a 1 in that position
                o2_rating = o2_rating.iter().filter(|e| ((*e & bit) / bit) == 1).map(|e| *e).collect();
            } else {
                // otherwire filter to keep only entries that have a 0 in that pos, since 
                // that is the most common bit
                o2_rating = o2_rating.iter().filter(|e| ((*e & bit) / bit) == 0).map(|e| *e).collect();
            }
            // if we only have one remaining entry, this is our final rating
            if o2_rating.iter().count() < 2 {
                break;
            }
        }

        let mut co2_rating = entries.clone();
        for bit in MASKS {
            size = co2_rating.len() as i32;
            if co2_rating.iter().map(|e| (e & bit) / bit).sum::<i32>() as f32 >= (size as f32) / 2.0 {
                co2_rating = co2_rating.iter().filter(|e| ((*e & bit) / bit) == 0).map(|e| *e).collect();
            } else {
                co2_rating = co2_rating.iter().filter(|e| ((*e & bit) / bit) == 1).map(|e| *e).collect();
            }
            if co2_rating.iter().count() < 2 {
                break;
            }
        }
        println!("O2: {}, CO2: {}, product: {}", o2_rating[0], co2_rating[0], o2_rating[0]*co2_rating[0]);
    }
}
