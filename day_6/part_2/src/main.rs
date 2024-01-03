use std::ops::Range;

use nom::{
    bytes::complete::take_till,
    character::{
        complete::{digit1, newline, space0},
        is_digit,
    },
    multi::many1,
    sequence::{preceded, separated_pair},
    IResult,
};
use roots::find_roots_quadratic;

fn main() {
    let s = include_str!("../input.txt");
    println!("{}", process(s));
}

fn winning_range(time: u64, distance_to_beat: u64) -> Option<Range<u64>> {
    let roots_race = find_roots_quadratic(-1f64, time as f64, -(distance_to_beat as f64));
    match roots_race {
        roots::Roots::No(_) | roots::Roots::One(_) => None,
        roots::Roots::Two(two_roots) => {
            let start = two_roots[0].floor() as u64 + 1;
            let end = two_roots[1].ceil() as u64;
            Some(start..end)
        }
        _ => unreachable!(),
    }
}

fn process(input: &str) -> u64 {
    let (_, (time, dist)) = parse_lines(input).unwrap();
    let range = winning_range(time, dist).expect("winnable race");
    range.end - range.start
}

fn parse_number(input: &str) -> IResult<&str, u64> {
    let (input, num_vec) = many1(preceded(space0, digit1))(input)?;
    let n = num_vec
        .iter()
        .fold(String::new(), |acc, e| acc + e)
        .parse()
        .unwrap();
    Ok((input, n))
}

fn parse_line(input: &str) -> IResult<&str, u64> {
    preceded(take_till(|c| is_digit(c as u8)), parse_number)(input)
}

fn parse_lines(input: &str) -> IResult<&str, (u64, u64)> {
    separated_pair(parse_line, newline, parse_line)(input)
}

#[cfg(test)]
#[test]
fn example() {
    let e = "Time:      7  15   30
Distance:  9  40  200";
    assert_eq!(process(e), 71503);
}

#[cfg(test)]
#[test]
fn part_2() {
    let s = include_str!("../input.txt");
    assert_eq!(process(s), 35150181);
}
