use std::ops::Range;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_till1, take_while},
    character::complete::{digit1, newline, space0, space1},
    combinator::map_res,
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, terminated},
    IResult,
};

fn main() {
    dbg!(parse_almanac(EXAMPLE));
}

#[derive(Debug)]
struct MapLine {
    source: Range<usize>,
    dest: Range<usize>,
}

#[derive(Debug)]
struct MapTable {
    mappings: Vec<MapLine>,
}

#[derive(Debug)]
struct Almanac {
    map_tables: Vec<MapTable>,
    seeds: Vec<usize>,
}

fn parse_almanac(input: &str) -> IResult<&str, Almanac> {
    let mut table_parser = delimited(newline, parse_map_table, alt((tag("\n"), tag(""))));
    let (input, _) = terminated(
        take_till1(|c| c == ':'),
        take_while(|c| c == ':' || c == ' '),
    )(input)?;
    let (input, seeds) = separated_list1(space1, parse_usize)(input)?;
    let (input, map_tables) = many1(parse_map_table)(input)?;
    Ok((input, Almanac { map_tables, seeds }))
}

// 50 98 2\n    52 50 48
fn parse_map_table(input: &str) -> IResult<&str, MapTable> {
    let (input, _) = take_till1(|c| c == ':')(input)?;
    let line_parser = delimited(space0, parse_map_line, alt((tag("\n"), tag(""))));
    let (input, _) = tag(":")(input)?;
    let (input, _) = newline(input)?;
    let (input, mappings) = many1(line_parser)(input)?;
    Ok((input, MapTable { mappings }))
}

fn parse_map_line(input: &str) -> IResult<&str, MapLine> {
    let (input, dest_start) = parse_usize(input)?;
    let (input, _) = space1(input)?;
    let (input, source_start) = parse_usize(input)?;
    let (input, _) = space1(input)?;
    let (input, offset) = parse_usize(input)?;
    let source = source_start..source_start + offset;
    let dest = dest_start..dest_start + offset;
    Ok((input, MapLine { source, dest }))
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s_var: &str| s_var.parse::<usize>())(input)
}

const EXAMPLE: &str = "seeds: 79 14 55 13

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
56 93 4";
