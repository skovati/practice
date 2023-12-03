use std::str::{self, FromStr};
use std::ops::Not;
use std::cmp;

use nom::{
    IResult,
    character::complete::{alpha1, self, line_ending},
    sequence::{separated_pair, preceded},
    bytes::complete::tag,
    multi::separated_list1
};

#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue
}

impl FromStr for Color {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => Err(())
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Cube {
    amount: u32,
    color: Color
}

impl Cube {
    fn exceeds_limit(s: &Self) -> bool {
        match s.color {
            Color::Red => s.amount > 12,
            Color::Green => s.amount > 13,
            Color::Blue => s.amount > 14
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Handful(Vec<Cube>);

impl Handful {
    fn exceeds_limit(s: &Self) -> bool {
        s.0.iter().any(Cube::exceeds_limit)
    }
}

#[derive(Debug, PartialEq)]
pub struct Game {
    id: u32,
    handfuls: Vec<Handful>
}

pub fn parse_cube(input: &str) -> IResult<&str, Cube> {
    let (input, (amount, color)) =
        separated_pair(complete::u32, tag(" "), alpha1)(input)?;
    Ok((input, Cube {
        amount,
        color: color.parse().expect("unsupported color")
    }))
}

pub fn parse_handful(input: &str) -> IResult<&str, Handful> {
    let (input, cubes) =
        separated_list1(tag(", "), parse_cube)(input)?;
    Ok((input, Handful(cubes)))
}

pub fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, id) =
        preceded(tag("Game "), complete::u32)(input)?;
    let (input, handfuls) =
        preceded(tag(": "), separated_list1(tag("; "), parse_handful))(input)?;
    Ok((input, Game { id, handfuls }))

}

pub fn parse(input: &str) -> Vec<Game> {
    let (_, games) =
        separated_list1(line_ending, parse_game)(input)
        .expect("failed to parse games");
    games
}

pub fn solve_one(games: Vec<Game>) -> u32 {
    games.iter()
        .filter(|g| g.handfuls.iter()
            .any(Handful::exceeds_limit)
            .not())
        .map(|g| g.id)
        .sum()
}

#[test]
fn test_parse_cube() {
    let input = "3 blue";
    let actual = parse_cube(input);
    let expected = Ok(("", Cube { amount: 3, color: Color::Blue }));
    assert_eq!(expected, actual);
}

#[test]
fn test_parse_handful() {
    let input = "3 blue, 4 red, 1 green";
    let actual = parse_handful(input);
    let expected = Ok(("", Handful(vec![
        Cube { amount: 3, color: Color::Blue },
        Cube { amount: 4, color: Color::Red },
        Cube { amount: 1, color: Color::Green }
    ])));
    assert_eq!(expected, actual);
}

#[test]
fn test_parse_game() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
    let actual = parse_game(input);
    let expected = Ok(("", Game {
        id: 1,
        handfuls: vec![
            Handful(vec![
                Cube { amount: 3, color: Color::Blue },
                Cube { amount: 4, color: Color::Red }
            ]),
            Handful(vec![
                Cube { amount: 1, color: Color::Red },
                Cube { amount: 2, color: Color::Green },
                Cube { amount: 6, color: Color::Blue }
            ]),
            Handful(vec![ Cube { amount: 2, color: Color::Green } ])
        ]
    }));
    assert_eq!(expected, actual);
}

#[test]
fn part_one_example() {
    let input = "
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    ".trim_start();
    let answer = 8;
    assert_eq!(answer, solve_one(parse(input)));
}

#[test]
fn part_one() {
    let input = include_str!("inputs/day02");
    let answer = 2285;
    assert_eq!(answer, solve_one(parse(input)));
}
