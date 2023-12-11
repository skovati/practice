use nom::{
    IResult,
    character::complete::{self, newline, space1},
    sequence::{pair, delimited},
    bytes::complete::tag,
    multi::separated_list1
};

/****************************************
* DATA MODEL
****************************************/
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Race {
    duration: u64,
    record: u64
}

/****************************************
* PARSERS
****************************************/
pub fn parse_list(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(space1, complete::u64)(input)
}

pub fn parse(input: &str) -> Vec<Race> {
    let (input, durations) = delimited(
        pair(tag("Time:"), space1),
        parse_list,
        newline
    )(input).expect("error parsing durations");

    let (_, records) = delimited(
        pair(tag("Distance:"), space1),
        parse_list,
        newline
    )(input).expect("error parsing records");

    durations.iter()
        .zip(records)
        .map(|(d, r)| {
            Race {
                duration: *d,
                record: r
            }
        })
        .collect()
}

pub fn parse_two(input: &str) -> Vec<Race> {
    let (input, durations) = delimited(
        pair(tag("Time:"), space1),
        parse_list,
        newline
    )(input).expect("error parsing durations");

    let (_, records) = delimited(
        pair(tag("Distance:"), space1),
        parse_list,
        newline
    )(input).expect("error parsing records");

    let d: u64 = durations.iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse()
        .expect("error parsing duration");

    let r: u64 = records.iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse()
        .expect("error parsing duration");

    vec![Race {
        duration: d,
        record: r
    }]
}

/****************************************
* SOLVERS
****************************************/
pub fn solve_one(races: Vec<Race>) -> u64 {
    races.iter()
        .map(|r| {
            (0..r.duration).map(|x| {
                (r.duration - x) * x
            })
            .filter(|e| *e > r.record)
            .count() as u64
        })
        .product()
}

/****************************************
* TESTS
****************************************/
pub const EXAMPLE_INPUT: &str = "
Time:      7  15   30
Distance:  9  40  200
";

#[test]
fn test_parse_list() {
    let input = "7      15             30";
    let actual = parse_list(input);
    let expected = Ok(("", vec![7, 15, 30]));
    assert_eq!(expected, actual);
}

#[test]
fn test_parse() {
    let input = EXAMPLE_INPUT.trim_start();
    let actual = parse(input);
    let expected = vec![
        Race {
            duration: 7,
            record: 9
        },
        Race {
            duration: 15,
            record: 40
        },
        Race {
            duration: 30,
            record: 200
        },
    ];
    assert_eq!(expected, actual);
}

#[test]
fn part_one_example() {
    let answer = 288;
    let input = EXAMPLE_INPUT.trim_start();
    let actual = solve_one(parse(input));
    assert_eq!(answer, actual);
}

#[test]
fn part_one() {
    let answer = 140220;
    let input = include_str!("inputs/day06");
    let actual = solve_one(parse(input));
    assert_eq!(answer, actual);
}

#[test]
fn part_two_example() {
    let answer = 71503;
    let input = EXAMPLE_INPUT.trim_start();
    let actual = solve_one(parse_two(input));
    assert_eq!(answer, actual);
}

#[test]
fn part_two() {
    let answer = 39570185;
    let input = include_str!("inputs/day06");
    let actual = solve_one(parse_two(input));
    assert_eq!(answer, actual);
}
