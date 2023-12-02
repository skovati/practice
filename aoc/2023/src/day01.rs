use std::str;

pub fn parse_one(file: &str) -> Vec<u32> {
    file.lines()
        .map(|l| {
            let first = l
                .chars()
                .find_map(|c| c.to_digit(10))
                .unwrap();
            let last = l
                .chars()
                .rev() .find_map(|c| c.to_digit(10)) .unwrap();

            first * 10 + last
        })
        .collect()
}

pub fn solve(values: Vec<u32>) -> u32 {
    values.iter().sum()
}

pub fn parse_two(file: &str) -> Vec<u32> {
    let digits = vec![ "0","1","2","3","4","5","6","7","8","9" ];
    let words = vec![ "zero","one","two","three","four","five","six","seven","eight","nine" ];

    let mut values = Vec::new();

    for l in file.lines() {
        println!("line: {l}");
        let mut first_index = u32::MAX;
        let mut first_value = "";
        let mut last_index = 0;
        let mut last_value = "";

        for w in &words {
            if let Some(i) = l.find(w) {
                if let Some(li) = l.rfind(w) {
                    if i < first_index as usize {
                        first_index = i as u32;
                        let pos = words.iter().position(|e| w == e).unwrap();
                        first_value = digits.get(pos).unwrap();
                    }
                    if li >= last_index as usize {
                        last_index = li as u32;
                        let pos = words.iter().position(|e| w == e).unwrap();
                        last_value = digits.get(pos).unwrap();
                    }
                }
            }
        }

        for d in &digits {
            if let Some(i) = l.find(d) {
                if let Some(li) = l.rfind(d) {
                    if i < first_index as usize {
                        first_index = i as u32;
                        first_value = d;
                    }
                    if li >= last_index as usize {
                        last_index = li as u32;
                        last_value = d;
                    }
                }
            }
        }

        println!("first_index: {first_index}");
        println!("first_value: {first_value}");

        println!("last_index: {last_index}");
        println!("last_value: {last_value}");

        let mut dig_str = first_value.to_string();
        dig_str.push_str(last_value);

        values.push(dig_str.parse().unwrap())
    }
    values
}

#[test]
fn part_one_small() {
    let input =
    "1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet";
    let answer = 142;
    assert_eq!(solve(parse_one(input)), answer);
}

#[test]
fn part_one_big() {
    let input = include_str!("inputs/day01/1");
    let values = parse_one(input);
    let answer = solve(values);
    assert_eq!(answer, 55447);
}

#[test]
fn part_two_small() {
    let input =
    "two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen";
    let answer = 281;
    assert_eq!(solve(parse_two(input)), answer);
}


#[test]
fn part_two_multiple_instance_of_digits() {
    let input = "32skznxsevenone3";
    let answer = 33;
    assert_eq!(solve(parse_two(input)), answer);
}

#[test]
fn part_two_big() {
    let input = include_str!("inputs/day01/2");
    let answer = 54706;
    assert_eq!(solve(parse_two(input)), answer);
}
