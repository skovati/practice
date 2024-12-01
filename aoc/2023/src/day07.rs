use std::str::FromStr;

use nom::{
    IResult,
    character::complete::{self, newline, space1},
    sequence::{pair, delimited},
    bytes::complete::tag,
    multi::separated_list1
};

use itertools::Itertools;

/****************************************
* DATA MODEL
****************************************/
#[derive(Debug, PartialEq, Clone)]
pub struct Hand {
    bid: u32,
    cards: Vec<Card>
}

impl Hand {
    fn is_five_of_a_kind(&self) -> bool {
        self.cards.iter()
            .unique()
            .count() == 1
    }
    fn is_four_of_a_kind(&self) -> bool {
        self.cards.iter()
            .map(|card| self.cards.iter().filter(|&x| x == card).count())
            .any(|count| count == 4)
    }
    fn is_three_of_a_kind(&self) -> bool {
        let counts = self.cards.iter()
            .map(|card| self.cards.iter().filter(|&x| x == card).count())
            .collect_vec();

        counts.iter().any(|&count| count == 3) &&
        counts.iter().any(|&count| count == 1)
    }
}


#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy, Hash)]
pub enum Card {
    Ace = 14,
    King = 13,
    Queen = 12,
    Jack = 11,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Card::Ace),
            "K" => Ok(Card::King),
            "Q" => Ok(Card::Queen),
            "J" => Ok(Card::Jack),
            "T" => Ok(Card::Ten),
            "9" => Ok(Card::Nine),
            "8" => Ok(Card::Eight),
            "7" => Ok(Card::Seven),
            "6" => Ok(Card::Six),
            "5" => Ok(Card::Five),
            "4" => Ok(Card::Four),
            "3" => Ok(Card::Three),
            "2" => Ok(Card::Two),
            _ => panic!("unsupported card")
        }
    }
}

/****************************************
* PARSERS
****************************************/
pub fn parse_list(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(space1, complete::u64)(input)
}


/****************************************
* SOLVERS
****************************************/
pub fn solve_one(hands: Vec<Hand>) -> u64 {
    0
}

/****************************************
* TESTS
****************************************/
pub const EXAMPLE_INPUT: &str = "
Time:      7  15   30
Distance:  9  40  200
";

#[test]
fn test_card_ordering() {
    assert!(Card::Four > Card::Two);
    assert!(Card::Ace > Card::Two);
    assert!(Card::Five > Card::Four);
}

#[test]
fn test_is_five_of_a_kind() {
    assert!(Hand::is_five_of_a_kind(&Hand {
        bid: 0,
        cards: vec![Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Ace]
    }))
}

#[test]
fn test_is_four_of_a_kind() {
    assert!(Hand::is_four_of_a_kind(&Hand {
        bid: 0,
        cards: vec![Card::Ace, Card::Two, Card::Ace, Card::Ace, Card::Ace]
    }))
}

#[test]
fn test_is_three_of_a_kind() {
    assert!(Hand::is_three_of_a_kind(&Hand {
        bid: 0,
        cards: vec![Card::Ace, Card::Two, Card::Ace, Card::Three, Card::Ace]
    }))
}

#[test]
#[ignore]
fn test_parse_list() {
    let input = "7      15             30";
    let actual = parse_list(input);
    let expected = Ok(("", vec![7, 15, 30]));
    assert_eq!(expected, actual);
}

