use std::fs;

fn main() {
    let input: Vec<i32> = fs::read_to_string("input/day06.txt")
        .expect("can't read from file!")
        .trim()
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    let mut school = input.clone();
    // println!("initial: {:?}", school);
    for _day in 1..81 {
        let mut to_add = 0;
        school.iter_mut().for_each(|f| {
            *f -= 1;
            if *f < 0 {
                to_add += 1;
                *f = 6;
            }
        });
        for _ in 0..to_add {
            school.push(8);
        }
        // println!("after day {}: {:?}", day, school);
    }

    let total = school.len();

    println!("total: {}", total);
}
