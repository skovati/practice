use nom::{
    IResult,
    character::complete::{self, char, newline, space1},
    sequence::{terminated, pair, delimited, tuple, preceded},
    bytes::complete::{tag, take_until},
    multi::{separated_list1, many0}
};

/****************************************
* DATA MODEL
****************************************/
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Race {
    duration: u32,
    record: u32
}

/****************************************
* PARSERS
****************************************/
pub fn parse_list(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(space1, complete::u32)(input)
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
const EXAMPLE_INPUT: &str = "
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
