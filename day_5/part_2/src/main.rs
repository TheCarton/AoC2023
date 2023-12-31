use std::ops::Range;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_till1, take_while},
    character::complete::{digit1, newline, space0, space1},
    combinator::{map, map_res},
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair, terminated},
    IResult,
};

fn main() {
    let e = "seeds: 79 14 55 13

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
    let s = include_str!("../input.txt");
    println!("{}", process(e));
}

#[derive(Debug)]
struct Almanac {
    map_tables: Vec<MapTable>,
    seeds: Vec<Range<usize>>,
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
fn overlap(x: &Range<usize>, y: &Range<usize>) -> bool {
    x.end > y.start && y.end > x.start
}

#[derive(Debug)]
enum RangeRelation {
    Disjoint,
    EqualRanges,
    SeedsAreProperSubset,
    SourceIsProperSubset,
    IntersectionSeedsLeft,
    IntersectionSeedsRight,
}

fn range_relation(seed_range: &Range<usize>, source_range: &Range<usize>) -> RangeRelation {
    if !overlap(seed_range, source_range) {
        RangeRelation::Disjoint
    } else if seed_range == source_range {
        RangeRelation::EqualRanges
    } else if source_range.contains(&seed_range.start) && source_range.contains(&seed_range.end) {
        RangeRelation::SeedsAreProperSubset
    } else if seed_range.contains(&source_range.start) && seed_range.contains(&source_range.end) {
        RangeRelation::SourceIsProperSubset
    } else if source_range.contains(&seed_range.end) {
        RangeRelation::IntersectionSeedsLeft
    } else {
        RangeRelation::IntersectionSeedsRight
    }
}

fn map_seed_with_map_line(seed_range: &Range<usize>, map_line: &MapLine) -> Range<usize> {
    let offset = map_line.dest.start as i64 - map_line.source.start as i64;
    let new_start = (seed_range.start as i64 + offset) as usize;
    let new_end = (seed_range.end as i64 + offset) as usize;
    let r = new_start..new_end;
    dbg!(r);
    new_start..new_end
}

// 79..92 seed range
// 50..98 soil source range, so the entire seed range.
// 52..100 soil dest range
// seed 79 goes to 81, seed 92 goes to 94
// seed 82 goes to soil 84
fn map_seeds(mut seed_ranges: Vec<Range<usize>>, map_table: &MapTable) -> Vec<Range<usize>> {
    let mut dest_seeds = Vec::new();
    while let Some(seed_range) = seed_ranges.pop() {
        dbg!(&seed_range);
        let mut seed_range_matched_a_source = false;
        for map_line in map_table.mappings.iter() {
            dbg!(map_line);
            match range_relation(&seed_range, &map_line.source) {
                RangeRelation::Disjoint => {
                    dbg!("disjoint");
                }
                RangeRelation::EqualRanges => {
                    seed_range_matched_a_source = true;
                    dbg!("equal");
                    dest_seeds.push(map_line.dest.clone())
                }
                RangeRelation::SeedsAreProperSubset => {
                    dbg!("seeds are proper subset");
                    seed_range_matched_a_source = true;
                    let new_dest_seed = map_seed_with_map_line(&seed_range, &map_line);
                    dest_seeds.push(new_dest_seed);
                }
                RangeRelation::SourceIsProperSubset => {
                    dbg!("source is proper subset");
                    seed_range_matched_a_source = true;
                    let left_remainder = seed_range.start..map_line.source.start;
                    dbg!(&left_remainder);
                    let right_remainder = map_line.source.end..seed_range.end;
                    dbg!(&right_remainder);

                    if !left_remainder.is_empty() {
                        seed_ranges.push(left_remainder);
                    }

                    if !right_remainder.is_empty() {
                        seed_ranges.push(right_remainder);
                    }

                    dest_seeds.push(map_line.dest.clone());
                }
                RangeRelation::IntersectionSeedsLeft => {
                    dbg!("intersect left");
                    seed_range_matched_a_source = true;
                    let remainder = seed_range.start..map_line.source.start;
                    dbg!(&remainder);
                    if !remainder.is_empty() {
                        seed_ranges.push(remainder);
                    }

                    let new_dest_seed = map_seed_with_map_line(&seed_range, &map_line);
                    dest_seeds.push(new_dest_seed);
                }
                RangeRelation::IntersectionSeedsRight => {
                    dbg!("intersect right");
                    seed_range_matched_a_source = true;
                    let remainder = map_line.source.end..seed_range.end;
                    dbg!(&remainder);
                    if !remainder.is_empty() {
                        seed_ranges.push(remainder);
                    }

                    let new_dest_seed = map_seed_with_map_line(&seed_range, &map_line);
                    dest_seeds.push(new_dest_seed);
                }
            }
        }
        if !seed_range_matched_a_source {
            dest_seeds.push(seed_range.clone());
        }
    }
    dbg!(&dest_seeds);
    dest_seeds
}

fn process(input: &str) -> usize {
    let (_, almanac) = parse_almanac(input).unwrap();
    let seeds = almanac
        .map_tables
        .iter()
        .fold(almanac.seeds, |seed_range, map_table| {
            let acc = map_seeds(seed_range, map_table);
            dbg!(&acc);
            acc
        });
    println!("{:?}", seeds);
    seeds
        .iter()
        .map(|seed_range| seed_range.start)
        .min()
        .expect("seed list should not be empty")
}

fn parse_almanac(input: &str) -> IResult<&str, Almanac> {
    let parse_seed_offset = separated_pair(parse_usize, space1, parse_usize);
    let parse_seed_range = map(parse_seed_offset, |(start, offset)| start..start + offset);
    let (input, _) = terminated(
        take_till1(|c| c == ':'),
        take_while(|c| c == ':' || c == ' '),
    )(input)?;
    let (input, seeds) = separated_list1(space1, parse_seed_range)(input)?;
    let (input, map_tables) = many1(parse_map_table)(input)?;
    Ok((input, Almanac { map_tables, seeds }))
}

fn parse_map_table(input: &str) -> IResult<&str, MapTable> {
    let line_parser = delimited(space0, parse_map_line, alt((tag("\n"), tag(""))));

    let (input, _) = take_till1(|c| c == ':')(input)?;
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

#[cfg(test)]
#[test]
fn example() {
    let s = "seeds: 79 14 55 13

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
    assert_eq!(process(s), 46);
}
