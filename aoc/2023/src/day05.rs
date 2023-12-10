use nom::{
    IResult,
    character::complete::{self, char, newline},
    sequence::{terminated, pair, delimited},
    bytes::complete::{tag, take_until},
    multi::{separated_list1, many0}
};

#[derive(Debug, PartialEq, Clone)]
pub struct MapRange {
    destination: u64,
    source: u64,
    range: u64,
}

impl MapRange {
    fn range_map(&self, num: u64) -> u64 {
        // if the number is within our source range
        if num >= self.source && num < self.source + self.range {
            // map it to the destination range
            num - self.source + self.destination
        } else {
            // otherwise, it stays the same
            num
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
        for range_map in &self.ranges {
            let res = range_map.range_map(num);
            if res != num {
                return res
            }
        }
        num
    }
}

pub fn parse_seeds(input: &str) -> IResult<&str, Vec<u64>> {
    delimited(
        tag("seeds: "),
        separated_list1(char(' '), complete::u64),
        many0(newline)
    )(input)
}

pub fn parse_map_range(input: &str) -> IResult<&str, MapRange> {
    let (input, nums) = separated_list1(char(' '), complete::u64)(input)?;
    let mut iter = nums.into_iter();
    Ok((input, MapRange {
        destination: iter.next().expect("error parsing range"),
        source: iter.next().expect("error parsing range"),
        range: iter.next().expect("error parsing range"),
    }))
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

pub fn solve_one(seeds: Vec<u64>, maps: Vec<Map>) -> u64 {
    // for each category map
    *maps.iter()
        // run our input seeds through each
        .fold(seeds, |acc, m| {
            acc.iter()
                .map(|a| m.range_map(*a))
                .collect::<Vec<u64>>()
        })
        .iter()
        .min()
        .expect("no minimum found")
}

#[test]
fn test_parse_seeds() {
    let input = "seeds: 41 48 83 86 17";
    let actual = parse_seeds(input);
    let expected = Ok(("", vec![41, 48, 83, 86, 17]));
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
    let input = "
seeds: 79 14 55 13

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
".trim_start();
    let answer = 35;
    let (seeds, maps) = parse(input);
    assert_eq!(answer, solve_one(seeds, maps));
}

#[test]
fn part_one() {
    let input = include_str!("inputs/day05");
    let answer = 322500873;
    let (seeds, maps) = parse(input);
    assert_eq!(answer, solve_one(seeds, maps));
}
