use std::collections::BTreeMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, newline, space1},
    combinator::{map, map_res},
    multi::{fold_many1, many1},
    sequence::{delimited, preceded, separated_pair, terminated},
    Err, IResult,
};

fn main() {
    dbg!(parse_main(EXAMPLE));
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    alt((
        map(tag("L"), |_| Direction::L),
        map(tag("R"), |_| Direction::R),
    ))(input)
}

fn parse_all_directions(input: &str) -> IResult<&str, Vec<Direction>> {
    many1(parse_direction)(input)
}

fn parse_mapkey(input: &str) -> IResult<&str, MapKey> {
    map_res(alpha1, |s: &str| MapKey::try_from(s))(input)
}

fn parse_mapnode(input: &str) -> IResult<&str, (MapKey, MapNode)> {
    let parse_pair = separated_pair(parse_mapkey, tag(", "), parse_mapkey);
    let (input, name) = terminated(parse_mapkey, tag(" = "))(input)?;

    let (input, (left, right)) = delimited(tag("("), parse_pair, tag(")"))(input)?;

    Ok((input, (name, MapNode { left, right })))
}

fn parse_tree(input: &str) -> IResult<&str, BTreeMap<MapKey, MapNode>> {
    fold_many1(
        terminated(parse_mapnode, alt((line_ending, tag("")))),
        BTreeMap::new,
        |mut acc: BTreeMap<MapKey, MapNode>, item| {
            acc.insert(item.0, item.1);
            acc
        },
    )(input)
}

fn parse_main(input: &str) -> IResult<&str, (Vec<Direction>, BTreeMap<MapKey, MapNode>)> {
    let (input, directions) = terminated(parse_all_directions, newline)(input)?;
    let (input, tree_map) = preceded(newline, parse_tree)(input)?;
    Ok((input, (directions, tree_map)))
}

const EXAMPLE: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

#[derive(Clone, Copy, Debug)]
enum Direction {
    L,
    R,
}

#[derive(Debug, Ord, PartialOrd, PartialEq, Eq)]
struct MapKey<'a> {
    key: &'a str,
}

impl<'a> TryFrom<&'a str> for MapKey<'a> {
    type Error = Err<&'a str>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        if value.len() == 3 {
            Ok(MapKey { key: value })
        } else {
            Err(nom::Err::Error(value))
        }
    }
}
#[derive(Debug)]
struct MapNode<'a> {
    left: MapKey<'a>,
    right: MapKey<'a>,
}
