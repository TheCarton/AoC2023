use std::collections::BTreeMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, newline},
    combinator::{map, map_res},
    multi::{fold_many1, many1},
    sequence::{delimited, preceded, separated_pair, terminated},
    Err, IResult,
};

fn main() {
    let s = include_str!("../input.txt");
    println!("{}", process(s));
}

struct MapIter<'a> {
    directions: &'a Vec<Direction>,
    map: &'a BTreeMap<MapKey<'a>, MapNode<'a>>,
    current_node: MapNode<'a>,
    target_key: MapKey<'a>,
    index: usize,
}

impl<'a> MapIter<'a> {
    fn new(
        directions: &'a Vec<Direction>,
        map: &'a BTreeMap<MapKey<'a>, MapNode<'a>>,
    ) -> MapIter<'a> {
        MapIter {
            directions,
            map,
            current_node: *map.get(&MapKey { key: "AAA" }).unwrap(),
            target_key: MapKey { key: "ZZZ" },
            index: 0,
        }
    }
    fn next_direction(&mut self) -> Direction {
        let i = self.index;
        self.index = (self.index + 1) % self.directions.len();
        self.directions[i]
    }
}

impl<'a> Iterator for MapIter<'a> {
    type Item = MapKey<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let next_key = match self.next_direction() {
            Direction::L => self.current_node.left,
            Direction::R => self.current_node.right,
        };
        if next_key == self.target_key {
            return None;
        }
        if let Some(next_node) = self.map.get(&next_key) {
            self.current_node = *next_node;
            Some(next_key)
        } else {
            None
        }
    }
}

fn process(input: &str) -> u32 {
    let (_, (directions, map)) = parse_main(input).unwrap();
    let it = MapIter::new(&directions, &map);
    it.filter(|&key| key != MapKey { key: "ZZZ" }).count() as u32 + 1
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

const EXAMPLE_1: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

const EXAMPLE_2: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

#[derive(Clone, Copy, Debug)]
enum Direction {
    L,
    R,
}

#[derive(Debug, Ord, PartialOrd, PartialEq, Eq, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
struct MapNode<'a> {
    left: MapKey<'a>,
    right: MapKey<'a>,
}

#[cfg(test)]
#[test]
fn example_1() {
    assert_eq!(process(EXAMPLE_1), 6);
}

#[cfg(test)]
#[test]
fn example_2() {
    assert_eq!(process(EXAMPLE_2), 2);
}

#[cfg(test)]
#[test]
fn part_1() {
    let s = include_str!("../input.txt");
    assert_eq!(process(s), 21883);
}
