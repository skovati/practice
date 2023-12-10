use nom::{
    IResult,
    character::complete::{self, char, newline, space1},
    sequence::{terminated, pair, delimited, tuple},
    bytes::complete::{tag, take_until},
    multi::{separated_list1, many0}
};

use rayon::prelude::*;

/****************************************
* DATA MODEL
****************************************/
#[derive(Debug, PartialEq, Clone)]
pub struct MapRange {
    destination: u64,
    source: u64,
    range: u64,
}

impl MapRange {
    fn try_convert(&self, num: u64) -> Option<u64> {
        // if the number is within our source range
        if num >= self.source && num < self.source + self.range {
            // map it to the destination range
            Some(num - self.source + self.destination)
        } else {
            // otherwise, it stays the same
            None
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Map {
    category: String,
    ranges: Vec<MapRange>
}

impl Map {
    fn range_map(&self, num: u64) -> u64 {
        self.ranges.iter()
            // short circuit on the first successful category mapping
            .find_map(|r| r.try_convert(num))
            .unwrap_or(num)
    }
}

/****************************************
* PARSERS
****************************************/
pub fn parse_seeds(input: &str) -> IResult<&str, Vec<u64>> {
    delimited(
        tag("seeds: "),
        separated_list1(char(' '), complete::u64),
        many0(newline)
    )(input)
}

pub fn parse_seeds_as_range(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, pairs) = delimited(
        tag("seeds: "),
        separated_list1(space1, tuple((complete::u64, space1, complete::u64))),
        many0(newline)
    )(input)?;

    Ok((input, pairs.par_iter()
        .flat_map(|(start, _, range)| {
            let bound = start + range;
            *start..bound
        })
        .collect()
    ))
}

pub fn parse_map_range(input: &str) -> IResult<&str, MapRange> {
    let (input, nums) = separated_list1(char(' '), complete::u64)(input)?;
    if let [destination, source, range] = nums[..3] {
        Ok((input, MapRange { destination, source, range }))
    } else {
        panic!("can't parse map range")
    }
}

pub fn parse_map(input: &str) -> IResult<&str, Map> {
    let (input, category) = terminated(
        take_until(" map:"),
        pair(tag(" map:"), newline)
    )(input)?;
    let (input, ranges) = separated_list1(newline, parse_map_range)(input)?;

    Ok((input, Map {
        category: category.to_string(),
        ranges
    }))

}

pub fn parse_maps(input: &str) -> IResult<&str, Vec<Map>> {
    let (input, maps) = separated_list1(
        pair(newline, newline), parse_map
    )(input)?;
    Ok((input, maps))
}

pub fn parse(input: &str) -> (Vec<u64>, Vec<Map>) {
    let (input, seeds) = parse_seeds(input).expect("error parsing seeds");
    let (_, maps) = parse_maps(input).expect("error parsing maps");
    (seeds, maps)
}

pub fn parse_two(input: &str) -> (Vec<u64>, Vec<Map>) {
    let (input, seeds) = parse_seeds_as_range(input).expect("error parsing seeds");
    let (_, maps) = parse_maps(input).expect("error parsing maps");
    (seeds, maps)
}

/****************************************
* SOLVERS
****************************************/
pub fn solve_one(seeds: Vec<u64>, maps: Vec<Map>) -> u64 {
    // for each category map
    *maps.iter()
        // run our input seeds through each range
        .fold(seeds, |acc, m| {
            acc.par_iter()
                .map(|a| m.range_map(*a))
                .collect()
        })
        .par_iter()
        .min()
        .expect("no minimum found")
}

/****************************************
* TESTS
****************************************/

const EXAMPLE_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

#[test]
fn test_parse_seeds() {
    let input = "seeds: 41 48 83 86 17";
    let actual = parse_seeds(input);
    let expected = Ok(("", vec![41, 48, 83, 86, 17]));
    assert_eq!(expected, actual);
}

#[test]
fn test_parse_seeds_as_range() {
    let input = "seeds: 41 3 83 5";
    let actual = parse_seeds_as_range(input);
    let expected = Ok(("", vec![41, 42, 43, 83, 84, 85, 86, 87]));
    assert_eq!(expected, actual);
}

#[test]
fn test_parse_map_range() {
    let input = "41 48 83";
    let actual = parse_map_range(input);
    let expected = Ok(("", MapRange {
        destination: 41,
        source: 48,
        range: 83
    }));
    assert_eq!(expected, actual);
}

#[test]
fn test_parse_map() {
    let input =
"seed-to-soil map:
50 98 2
52 50 48".trim_start();
    let actual = parse_map(input);
    let expected = Ok(("", Map {
        category: "seed-to-soil".to_string(),
        ranges: vec![
            MapRange {
                destination: 50,
                source: 98,
                range: 2
            }, 
            MapRange {
                destination: 52,
                source: 50,
                range: 48
            }, 
        ]
    }));
    assert_eq!(expected, actual);
}

#[test]
fn test_parse_maps() {
    let input = "
seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

".trim();
    let actual = parse_maps(input);
    let expected = Ok(("", vec![
        Map {
            category: "seed-to-soil".to_string(),
            ranges: vec![
                MapRange {
                    destination: 50,
                    source: 98,
                    range: 2
                }, 
                MapRange {
                    destination: 52,
                    source: 50,
                    range: 48
                }, 
            ]
        },
        Map {
            category: "soil-to-fertilizer".to_string(),
            ranges: vec![
                MapRange {
                    destination: 0,
                    source: 15,
                    range: 37
                }, 
                MapRange {
                    destination: 37,
                    source: 52,
                    range: 2
                }, 
                MapRange {
                    destination: 39,
                    source: 0,
                    range: 15
                }, 
            ]
        },
    ]));
    assert_eq!(expected, actual);
}

#[test]
fn test_parse_seeds_maps() {
    let input = "
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

".trim();
    let actual = parse(input);
    let expected = (vec![79, 14, 55, 13], vec![
        Map {
            category: "seed-to-soil".to_string(),
            ranges: vec![
                MapRange {
                    destination: 50,
                    source: 98,
                    range: 2
                }, 
                MapRange {
                    destination: 52,
                    source: 50,
                    range: 48
                }, 
            ]
        },
    ]);
    assert_eq!(expected, actual);
}

#[test]
fn part_one_example() {
    let answer = 35;
    let (seeds, maps) = parse(EXAMPLE_INPUT);
    assert_eq!(answer, solve_one(seeds, maps));
}

#[test]
fn part_one() {
    let input = include_str!("inputs/day05");
    let answer = 322500873;
    let (seeds, maps) = parse(input);
    assert_eq!(answer, solve_one(seeds, maps));
}

#[test]
fn part_two_example() {
    let answer = 46;
    let (seeds, maps) = parse_two(EXAMPLE_INPUT);
    assert_eq!(answer, solve_one(seeds, maps));
}

#[test]
#[ignore]
fn part_two() {
    let input = include_str!("inputs/day05");
    let answer = 108956227;
    let (seeds, maps) = parse_two(input);
    assert_eq!(answer, solve_one(seeds, maps));
}
