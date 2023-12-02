use std::fs;

// max num determined by (max timer a fish can have) + (1 to represent -1 state)
// + (1 since we loop < MAX_NUM)
const MAX_NUM: usize = 10;

#[allow(dead_code)]
fn print_school(s: &Vec<usize>) {
    for i in 1..s.len() {
        print!("{}", i-1);
    }
    println!();
    for i in 1..s.len() {
        print!("{}", s[i]);
    }
    println!();
}

fn count_fish(init: &Vec<usize>, days: usize) -> usize {
    // we use an array where indicies 1-9 represent the number of fish that have
    // timers 0-8 respectfully. this is done so we have room to represent a "-1" state,
    // which is fish that were 0 the previous day and therefore are due to reproduce the current
    // day and then turn into a 6. we do this so we can stick to using usize and a vector, instead
    // of a HashMap<isize>, which would cut our representable fish size in half.
    let mut school: Vec<usize> = vec![0; MAX_NUM];
    // seed initial state
    (0..MAX_NUM-1).for_each(|i| {
        school[i+1] = init
            .iter()
            .filter(|n| **n == i)
            .count();
    });

    for _day in 0..days {
        for i in 1..MAX_NUM {
            // decrement each fish timer
            school[i-1] = school[i];
        }
        // add 6's for every fish that had a timer of 0 the previous day
        // again, school[0] represents the fish that had a timer of 0 the previous day
        // which are the ones that need to reproduce and turn into a 6
        school[7] += school[0];
        // and add new 8's that the fish birthed
        school[9] = school[0];
    }

    // the total number of fish in the school is simply the sum of every element
    // in the vector except index 0, since that holds the number of fish that reproduced the last
    // day, which is already counted at index 7, so we don't want to double count.
    school.iter().skip(1).sum()
}

fn main() {
    // parse input file into vector
    let input: Vec<usize> = fs::read_to_string("input/day06.txt")
        .expect("can't read from file!")
        .trim()
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect();
    
    // part 1
    println!("fish after 80 days: {}", count_fish(&input, 80));

    // part 2
    println!("fish after 256 days: {}", count_fish(&input, 256));
}

#[test]
fn small_input_80() {
    let initial_state = &vec![3,4,3,1,2];
    assert_eq!(count_fish(initial_state, 80), 5934);
}

#[test]
fn small_input_256() {
    let initial_state = &vec![3,4,3,1,2];
    assert_eq!(count_fish(initial_state, 256), 26984457539);
}

#[test]
fn large_input_80() {
    let initial_state = &vec![5,1,1,3,1,1,5,1,2,1,5,2,5,1,1,1,4,1,1,5,1,1,4,1,1,1,3,5,1,1,1,1,1,1,1,1,1,4,4,4,1,1,1,1,1,4,1,1,1,1,1,5,1,1,1,4,1,1,1,1,1,3,1,1,4,1,4,1,1,2,3,1,1,1,1,4,1,2,2,1,1,1,1,1,1,3,1,1,1,1,1,2,1,1,1,1,1,1,1,4,4,1,4,2,1,1,1,1,1,4,3,1,1,1,1,2,1,1,1,2,1,1,3,1,1,1,2,1,1,1,3,1,3,1,1,1,1,1,1,1,1,1,3,1,1,1,1,3,1,1,1,1,1,1,2,1,1,2,3,1,2,1,1,4,1,1,5,3,1,1,1,2,4,1,1,2,4,2,1,1,1,1,1,1,1,2,1,1,1,1,1,1,1,1,4,3,1,2,1,2,1,5,1,2,1,1,5,1,1,1,1,1,1,2,2,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,4,1,1,1,1,1,3,1,1,5,1,1,1,1,5,1,4,1,1,1,4,1,3,4,1,4,1,1,1,1,1,1,1,1,1,3,5,1,3,1,1,1,1,4,1,5,3,1,1,1,1,1,5,1,1,1,2,2];
    assert_eq!(count_fish(initial_state, 80), 394994);
}

#[test]
fn large_input_256() {
    let initial_state = &vec![5,1,1,3,1,1,5,1,2,1,5,2,5,1,1,1,4,1,1,5,1,1,4,1,1,1,3,5,1,1,1,1,1,1,1,1,1,4,4,4,1,1,1,1,1,4,1,1,1,1,1,5,1,1,1,4,1,1,1,1,1,3,1,1,4,1,4,1,1,2,3,1,1,1,1,4,1,2,2,1,1,1,1,1,1,3,1,1,1,1,1,2,1,1,1,1,1,1,1,4,4,1,4,2,1,1,1,1,1,4,3,1,1,1,1,2,1,1,1,2,1,1,3,1,1,1,2,1,1,1,3,1,3,1,1,1,1,1,1,1,1,1,3,1,1,1,1,3,1,1,1,1,1,1,2,1,1,2,3,1,2,1,1,4,1,1,5,3,1,1,1,2,4,1,1,2,4,2,1,1,1,1,1,1,1,2,1,1,1,1,1,1,1,1,4,3,1,2,1,2,1,5,1,2,1,1,5,1,1,1,1,1,1,2,2,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,4,1,1,1,1,1,3,1,1,5,1,1,1,1,5,1,4,1,1,1,4,1,3,4,1,4,1,1,1,1,1,1,1,1,1,3,5,1,3,1,1,1,1,4,1,5,3,1,1,1,1,1,5,1,1,1,2,2];
    assert_eq!(count_fish(initial_state, 256), 1765974267455);
}
