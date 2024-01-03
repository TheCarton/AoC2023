use std::ops::Range;

use nom::{
    bytes::complete::take_till,
    character::{
        complete::{digit1, space1},
        is_digit,
    },
    combinator::map_res,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};
use roots::find_roots_quadratic;

fn main() {
    let s = include_str!("../input.txt");
    println!("{}", process(s));
}

fn winning_range(time: u32, distance_to_beat: u32) -> Option<Range<u32>> {
    let roots_race = find_roots_quadratic(-1f32, time as f32, -(distance_to_beat as f32));
    match roots_race {
        roots::Roots::No(_) | roots::Roots::One(_) => None,
        roots::Roots::Two(two_roots) => {
            let start = two_roots[0].floor() as u32 + 1;
            let end = two_roots[1].ceil() as u32;
            Some(start..end)
        }
        _ => unreachable!(),
    }
}

fn process(input: &str) -> u32 {
    let (_, (times, dists)) = parse_time_and_dist(input).unwrap();
    times
        .iter()
        .zip(dists)
        .map(|(time, dist)| {
            let win_range = winning_range(*time, dist).expect("all races should be winnable.");
            win_range.end - win_range.start
        })
        .product()
}

fn take_till_first_num(input: &str) -> IResult<&str, &str> {
    take_till(|c| is_digit(c as u8))(input)
}

fn parse_u32(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

fn parse_time_and_dist(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    let (input, (_, times)) =
        tuple((take_till_first_num, separated_list1(space1, parse_u32)))(input)?;
    let (input, (_, dists)) =
        tuple((take_till_first_num, separated_list1(space1, parse_u32)))(input)?;
    Ok((input, (times, dists)))
}

#[cfg(test)]
#[test]
fn example() {
    let e = "Time:      7  15   30
Distance:  9  40  200";
    assert_eq!(process(e), 288);
}

#[test]
fn roots_for_one_race() {
    let r = winning_range(7, 9);
    assert_eq!(r, Some(2..6));
}

#[test]
fn part_1() {
    let s = include_str!("../input.txt");
    assert_eq!(process(s), 293046);
}
