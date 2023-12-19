use nom::branch::alt;
use nom::bytes::complete::{tag, take_till, take_while};
use nom::character::complete::{space0, space1, u32};
use nom::IResult;

use nom::multi::many1;
use nom::sequence::{delimited, terminated, tuple};

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", process(input));
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
    let (max_red, max_green, max_blue) = (12, 13, 14);
    input
        .lines()
        .filter_map(|line| {
            let (_, game) = parse_game(line).expect("valid game");
            let valid_game = game.rounds.iter().all(|round| {
                round.blocks.iter().all(|block| match block {
                    Block::Red(r) => r <= &max_red,
                    Block::Green(g) => g <= &max_green,
                    Block::Blue(b) => b <= &max_blue,
                })
            });
            if valid_game {
                Some(game.id)
            } else {
                None
            }
        })
        .sum()
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let mut header_parser = delimited(tag("Game "), u32, tag(": "));
    let (game_string, id) = header_parser(input)?;
    let (remaining, rounds) = many1(round)(game_string)?;
    Ok((remaining, Game { id, rounds }))
}

fn round(input: &str) -> IResult<&str, Round> {
    let mut round_parser = terminated(
        take_till(|c| c == ';' || c == '\n'),
        take_while(|c| c == ';' || c == ' '),
    );
    let block_parser = delimited(space0, alt((red, blue, green)), alt((tag(","), tag(""))));
    let (remaining, round_string) = round_parser(input)?;
    let (_, blocks) = many1(block_parser)(round_string)?;
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
    let s = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    assert_eq!(process(s), 8);
}

#[cfg(test)]
#[test]
fn part_1() {
    let s = include_str!("../input.txt");
    assert_eq!(process(s), 2551);
}
#[test]
fn debug_parse_input() {
    let s = include_str!("../input.txt");
    for l in s.lines() {
        let g = parse_game(l);
        dbg!(g);
    }
}

#[test]
fn debug_parse_example() {
    let s = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    for l in s.lines() {
        let g = parse_game(l);
        dbg!(g);
    }
}

#[test]
fn debug_parse_game() {
    let s = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
    let r = parse_game(s);
    dbg!(r);
}

#[test]
fn debug_parse_round() {
    let s = "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
    let r = round(s);
}
