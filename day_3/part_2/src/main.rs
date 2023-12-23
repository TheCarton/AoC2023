use std::collections::HashMap;

use glam::IVec2;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_till1},
    character::complete::{digit1, none_of, one_of},
    multi::many1,
    sequence::terminated,
    IResult,
};
use nom_locate::{position, LocatedSpan};
type Span<'a> = LocatedSpan<&'a str>;

fn main() {
    let s = include_str!("../input.txt");
    println!("{}", process(s));
}

#[derive(Debug)]
struct Number<'a> {
    n_str: &'a str,
    pos: Span<'a>,
}

impl<'a> Number<'a> {
    fn xy(&self) -> IVec2 {
        let x = self.pos.get_column() as i32 - 1;
        let y = self.pos.location_line() as i32 - 1;
        IVec2 { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DiagramChar {
    Symbol,
    Digit,
    Nothing,
}

#[derive(Debug)]
struct Diagram<'a> {
    rows: Vec<Vec<DiagramChar>>,
    numbers: Vec<Number<'a>>,
}

impl<'a> Diagram<'a> {
    fn get(&self, pos: IVec2) -> Option<DiagramChar> {
        let valid_x = pos.x >= 0 && pos.x < self.rows.first().unwrap().len() as i32;
        let valid_y = pos.y >= 0 && pos.y < self.rows.len() as i32;
        if valid_x && valid_y {
            Some(self.rows[pos.y as usize][pos.x as usize])
        } else {
            None
        }
    }
    fn get_mut(&mut self, pos: IVec2) -> Option<&mut DiagramChar> {
        let valid_x = pos.x >= 0 && pos.x < self.rows.first().unwrap().len() as i32;
        let valid_y = pos.y >= 0 && pos.y < self.rows.len() as i32;
        if valid_x && valid_y {
            Some(&mut self.rows[pos.y as usize][pos.x as usize])
        } else {
            None
        }
    }
}

fn parse_number<'a>(s: Span<'a>) -> IResult<Span, Number<'a>> {
    let (num, _) = take_till(|c: char| c.is_ascii_digit())(s)?;
    let (s, num) = digit1(num)?;

    let (_, pos) = position(num)?;
    Ok((s, Number { n_str: &num, pos }))
}

fn parse_diagram(input: &str) -> IResult<&str, Diagram> {
    let (_s, numbers) = many1(parse_number)(input.into()).unwrap();
    let (input, rows) = many1(parse_row)(input)?;
    Ok((input, Diagram { rows, numbers }))
}
fn parse_row(input: &str) -> IResult<&str, Vec<DiagramChar>> {
    let newline_or_empty = alt((tag("\n"), tag("")));
    let (input, row_string) = terminated(take_till1(|c| c == '\n'), newline_or_empty)(input)?;
    let (_, row) = many1(alt((parse_symbol, parse_dot, parse_digit)))(row_string)?;
    Ok((input, row))
}

fn parse_symbol(input: &str) -> IResult<&str, DiagramChar> {
    let (input, _) = one_of("*")(input)?;
    Ok((input, DiagramChar::Symbol))
}

fn parse_dot(input: &str) -> IResult<&str, DiagramChar> {
    let (input, _) = none_of("*0123456789")(input)?;
    Ok((input, DiagramChar::Nothing))
}

fn parse_digit(input: &str) -> IResult<&str, DiagramChar> {
    let (input, _) = one_of("0123456789")(input)?;
    Ok((input, DiagramChar::Digit))
}

struct SymbolTuple {
    adjacent_n: u8,
    ratio: u32,
}

impl SymbolTuple {
    fn new(adjacent_n: u8, ratio: u32) -> SymbolTuple {
        SymbolTuple { adjacent_n, ratio }
    }
}

fn process(input: &str) -> u32 {
    let (_, diagram) = parse_diagram(input).expect("valid input");
    let adj_map = diagram.numbers.iter().fold(
        HashMap::new(),
        |mut adj_map: HashMap<IVec2, SymbolTuple>, num| {
            let start_pos = num.xy();
            let end_pos = num.xy() + IVec2::new(num.n_str.len() as i32 - 1, 0);

            let west_border =
                (-1..=1).map(|delta_y| IVec2::new(start_pos.x - 1, delta_y + start_pos.y));
            let east_border =
                (-1..=1).map(|delta_y| IVec2::new(end_pos.x + 1, delta_y + end_pos.y));

            let north_border = (start_pos.x..=end_pos.x).map(|x| IVec2::new(x, start_pos.y - 1));
            let south_border = (start_pos.x..=end_pos.x).map(|x| IVec2::new(x, start_pos.y + 1));
            north_border
                .chain(south_border.chain(east_border.chain(west_border)))
                .for_each(|border_v| {
                    if diagram
                        .get(border_v)
                        .is_some_and(|c| c == DiagramChar::Symbol)
                    {
                        let part_number = num.n_str.parse::<u32>().unwrap();
                        if let Some(s_tuple) = adj_map.get_mut(&border_v) {
                            s_tuple.adjacent_n += 1;
                            s_tuple.ratio *= part_number;
                        } else {
                            adj_map.insert(border_v, SymbolTuple::new(1, part_number));
                        }
                    }
                });
            adj_map
        },
    );
    adj_map
        .into_values()
        .filter_map(|s_tuple| {
            if s_tuple.adjacent_n == 2 {
                Some(s_tuple.ratio)
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
#[test]
fn example_1() {
    let s = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    assert_eq!(467835, process(s));
}
