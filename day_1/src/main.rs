fn main() {
    let i = include_str!("../input1.txt");
    println!("{}", process(i));
}

fn process(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let mut it = l.chars().filter(|c| c.is_digit(10));
            let first = it.next().unwrap();
            let last = match it.last() {
                Some(d) => d,
                None => first,
            };
            let coords = format!("{}{}", first, last);
            coords.parse::<u32>().unwrap()
        })
        .sum()
}

#[cfg(test)]
#[test]
fn example_1() {
    let s = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";
    assert_eq!(process(&s), 142);
}
