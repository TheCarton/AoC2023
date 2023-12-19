fn main() {
    let s = include_str!("../input.txt");
    println!("{}", process(s));
}

fn match_spelled_out(s: &str) -> Option<u32> {
    let words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for j in 0..words.len() {
        if words[j].len() <= s.len() && &s[..words[j].len()] == words[j] {
            return Some(j as u32 + 1);
        }
    }
    None
}

fn process(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut it = (0..line.len()).filter_map(|i| {
                let c = line[i..].chars().next().unwrap();
                if c.is_digit(10) {
                    c.to_digit(10)
                } else {
                    match_spelled_out(&line[i..])
                }
            });
            let first = it.next().unwrap();
            let last = match it.last() {
                Some(d) => d,
                None => first,
            };
            format!("{first}{last}").parse::<u32>().unwrap()
        })
        .sum()
}

#[cfg(test)]
#[test]
fn example_1() {
    let s = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
    assert_eq!(process(s), 281);
}
