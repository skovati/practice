use std::collections::{BTreeSet, BTreeMap};

use nom::{
    IResult,
    character::complete::{self, char, line_ending},
    sequence::{separated_pair, preceded, delimited, tuple},
    bytes::complete::tag,
    multi::{separated_list1, many0, many1}
};

#[derive(Debug, PartialEq, Clone)]
pub struct Card {
    id: u32,
    winning: BTreeSet<u32>,
    contents: BTreeSet<u32>
}

impl Card {
    fn num_wins(&self) -> u32 {
        self.contents.intersection(&self.winning)
            .copied()
            .collect::<Vec<u32>>()
            .len() as u32
    }
}

pub fn manyspace(input: &str) -> IResult<&str, Vec<char>> {
    many0(char(' '))(input)
}

pub fn parse_list(input: &str) -> IResult<&str, BTreeSet<u32>> {
    let (input, nums) =
        delimited(
            manyspace,
            separated_list1(many1(char(' ')), complete::u32),
            manyspace
    )(input)?;

    Ok((input, nums.into_iter().collect()))
}

pub fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, id) = preceded(
        tuple(( tag("Card"), manyspace )),
        complete::u32
    )(input)?;

    let (input, _) = char(':')(input)?;

    let (input, (winning, contents)) = separated_pair(
        parse_list,
        tag("|"),
        parse_list
    )(input)?;

    Ok((input, Card {
        id,
        winning,
        contents
    }))
}

pub fn parse(input: &str) -> Vec<Card> {
    let (_, input) =
        separated_list1(line_ending, parse_card)(input)
        .expect("failed to parse stack");
    input
}

pub fn solve_one(cards: Vec<Card>) -> u32 {
    cards.iter()
        .map(Card::num_wins)
        .map(|wins| if wins == 0 {
                wins
            } else {
                2_u32.pow(wins - 1)
            })
        .sum()
}

pub fn solve_two(cards: Vec<Card>) -> u32 {
    let mut amount: BTreeMap<u32, u32> = BTreeMap::new();
    let mut total = 0;

    for (index, card) in cards.iter().enumerate() {
        // current card count is any previously added + original
        let curr_card_count = *amount.entry(index as u32).or_insert(0) + 1;

        // tally total number of cards
        total += curr_card_count;

        // increase card holdings based on wins
        for i in 1..=card.num_wins() {
            let r = amount.entry(index as u32 + i)
                .or_insert(0);
            *r += curr_card_count;
        }
    }
    total
}

#[test]
fn test_parse_list() {
    let input = "41 48 83  86   17";
    let actual = parse_list(input);
    let expected = Ok(("", BTreeSet::from([41, 48, 83, 86, 17])));
    assert_eq!(expected, actual);
}

#[test]
fn test_parse_card() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
    let actual = parse_card(input);
    let expected = Ok(("", Card {
        id: 1,
        winning: BTreeSet::from([41, 48, 83, 86, 17]),
        contents: BTreeSet::from([83, 86, 6, 31, 17, 9, 48, 53])
    }));
    assert_eq!(expected, actual);
}

#[test]
fn part_one_example() {
    let input = "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    ".trim_start();
    let answer = 13;
    assert_eq!(answer, solve_one(parse(input)));
}

#[test]
fn part_one() {
    let input = include_str!("inputs/day04");
    let answer = 25571;
    assert_eq!(answer, solve_one(parse(input)));
}

#[test]
fn part_two_example() {
    let input = "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    ".trim_start();
    let answer = 30;
    assert_eq!(answer, solve_two(parse(input)));
}

#[test]
fn part_two() {
    let input = include_str!("inputs/day04");
    let answer = 8805731;
    assert_eq!(answer, solve_two(parse(input)));
}
