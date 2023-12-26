use std::collections::HashSet;

use nom::{
    bytes::complete::{take_till1, take_while1},
    character::complete::{digit1, space1},
    combinator::map_res,
    multi::{many1, separated_list1},
    sequence::terminated,
    IResult,
};

fn main() {
    let s = include_str!("../input.txt");
    println!("{}", process(s));
}

#[derive(Debug)]
struct Card {
    win_set: HashSet<u32>,
    have_nums: Vec<u32>,
}

fn process(input: &str) -> u32 {
    let (_, cards) = parse_all_cards(input).unwrap();
    cards
        .iter()
        .map(|card| {
            let matching_n = card
                .have_nums
                .iter()
                .filter(|n| card.win_set.contains(n))
                .count() as u32;

            if matching_n <= 2 {
                matching_n
            } else {
                2u32.pow(matching_n - 1)
            }
        })
        .sum()
}

fn parse_all_cards(input: &str) -> IResult<&str, Vec<Card>> {
    many1(parse_card)(input)
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let parse_u32 = map_res(digit1, |s: &str| s.parse::<u32>());
    let mut parse_list_u32 = separated_list1(space1, parse_u32);

    let (input, _card) = terminated(
        take_till1(|c| c == ':'),
        take_while1(|c| c == ':' || c == ' '),
    )(input)?;

    let (input, win_nums) = parse_list_u32(input)?;
    let win_set: HashSet<u32> = win_nums.into_iter().collect();
    let (input, _delim) = take_while1(|c| c == '|' || c == ' ')(input)?;

    let (input, have_nums) = parse_list_u32(input)?;
    Ok((input, Card { win_set, have_nums }))
}

#[cfg(test)]
#[test]
fn example() {
    let example_s = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    assert_eq!(process(example_s), 13);
}

#[cfg(test)]
#[test]
fn part_1() {
    let s = include_str!("../input.txt");
    assert_eq!(process(s), 15205);
}
