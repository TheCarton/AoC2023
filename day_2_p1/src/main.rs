use nom::branch::alt;
use nom::bytes::complete::{tag, take_till, take_while};
use nom::character::complete::{line_ending, space0, space1, u32};
use nom::IResult;

use nom::combinator::eof;
use nom::multi::{fold_many0, many0, many1};
use nom::sequence::{delimited, preceded, terminated, tuple};

fn main() {
    debug_parse_game();
}

fn debug_parse_game() {
    let s = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
    let r = game(s);
    dbg!(r);
}

fn debug_parse_round() {
    let s = "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
    let r = round(s);
    dbg!(r);
}

#[derive(Debug, Clone, Copy)]
enum Block {
    Red(u32),
    Green(u32),
    Blue(u32),
}

#[derive(Debug)]
struct Round {
    blocks: Vec<Block>,
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

fn process(input: &str) -> u32 {
    todo!();
}

// "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
fn game(input: &str) -> IResult<&str, Game> {
    let mut header_parser = delimited(tag("Game "), u32, tag(": "));
    let (game_string, id) = header_parser(input)?;
    dbg!(game_string);
    dbg!(id);
    let (remaining, rounds) = many0(round)(game_string)?;
    dbg!(&rounds);
    Ok((remaining, Game { id, rounds }))
}
// "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

// doesn't return error when it should
fn round(input: &str) -> IResult<&str, Round> {
    dbg!(input);
    let mut round_parser = terminated(
        take_till(|c| c == ';' || c == '\n'),
        take_while(|c| c == ';' || c == ' '),
    );
    let block_parser = delimited(space0, alt((red, blue, green)), alt((tag(","), tag(""))));
    let (remaining, round_string) = round_parser(input)?;
    let (_, blocks) = many1(block_parser)(round_string)?;
    dbg!(&blocks);
    Ok((remaining, Round { blocks }))
}

fn red(input: &str) -> IResult<&str, Block> {
    let (remaining, n) = terminated(u32, tuple((space1, tag("red"))))(input)?;
    Ok((remaining, Block::Red(n)))
}

fn blue(input: &str) -> IResult<&str, Block> {
    let (remaining, n) = terminated(u32, tuple((space1, tag("blue"))))(input)?;
    Ok((remaining, Block::Blue(n)))
}
fn green(input: &str) -> IResult<&str, Block> {
    let (remaining, n) = terminated(u32, tuple((space1, tag("green"))))(input)?;
    Ok((remaining, Block::Green(n)))
}

#[cfg(test)]
#[test]
fn example_1() {
    let s = "
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    assert_eq!(process(s), 8);
}
