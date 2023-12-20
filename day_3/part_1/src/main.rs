use glam::IVec2;

use nom::{
    branch::alt,
    bytes::complete::{is_a, is_not, tag, take_till1, take_until1, take_while1},
    character::{
        complete::{digit0, digit1, newline, none_of, one_of},
        is_alphabetic, is_newline,
    },
    combinator::rest,
    multi::many1,
    sequence::{terminated, tuple},
    IResult,
};
use nom_locate::{position, LocatedSpan};
type Span<'a> = LocatedSpan<&'a str>;

fn main() {
    test_parse_diagram();
}

#[derive(Debug)]
struct Number<'a> {
    n: &'a str,
    pos: Span<'a>,
}

#[derive(Debug)]
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

fn parse_number<'a>(s: Span<'a>) -> IResult<Span, Number<'a>> {
    let (s, num) = digit1(s)?;
    let (s, pos) = position(num)?;
    Ok((s, Number { n: &num, pos }))
}

fn test_parse_diagram() {
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
    println!("{s}");
    dbg!(parse_diagram(s));
}

fn parse_diagram(input: &str) -> IResult<&str, Diagram> {
    dbg!(input);
    let (input, rows) = many1(parse_row)(input)?;
    Ok((
        input,
        Diagram {
            rows,
            numbers: vec![],
        },
    ))
}

fn test_parse_row() {
    let row = "617*......";
    dbg!(row);
    dbg!(parse_row(row));
}

fn parse_row(input: &str) -> IResult<&str, Vec<DiagramChar>> {
    let newline_or_empty = alt((tag("\n"), tag("")));
    let (input, row_string) = terminated(take_till1(|c| c == '\n'), newline_or_empty)(input)?;
    dbg!(row_string);
    let (_, row) = many1(alt((parse_symbol, parse_dot, parse_digit)))(row_string)?;
    Ok((input, row))
}

fn parse_symbol(input: &str) -> IResult<&str, DiagramChar> {
    let (input, _) = none_of(".0123456789")(input)?;
    Ok((input, DiagramChar::Symbol))
}

fn parse_dot(input: &str) -> IResult<&str, DiagramChar> {
    let (input, _) = one_of(".")(input)?;
    Ok((input, DiagramChar::Nothing))
}

fn parse_digit(input: &str) -> IResult<&str, DiagramChar> {
    let (input, _) = one_of("0123456789")(input)?;
    Ok((input, DiagramChar::Digit))
}

fn process(input: &str) -> u32 {
    todo!();
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
    assert_eq!(4361, process(s));
}
