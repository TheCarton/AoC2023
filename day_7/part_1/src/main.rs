use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline, space1},
    combinator::{map, map_res},
    multi::{fold_many1, separated_list1},
    sequence::separated_pair,
    IResult,
};

fn main() {
    let s = include_str!("../input.txt");
    println!("{}", process(s));
}

fn process(input: &str) -> u32 {
    let (_, mut hand_tuples) = parse_hand_tuples(input).unwrap();
    hand_tuples.sort_unstable();
    hand_tuples
        .iter()
        .enumerate()
        .map(|(i, hand_tuple)| (i as u32 + 1) * hand_tuple.bid)
        .sum()
}

#[derive(Debug, Eq)]
struct HandTuple {
    hand: Hand,
    bid: u32,
}

impl Ord for HandTuple {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand.cmp(&other.hand)
    }
}

impl PartialOrd for HandTuple {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HandTuple {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    hand_type: HandType,
    cards: [Card; 5],
}

impl Hand {
    fn new(cards: [Card; 5]) -> Hand {
        Hand {
            hand_type: strongest_hand_type(&cards),
            cards,
        }
    }
}

fn strongest_hand_type(cards: &[Card; 5]) -> HandType {
    let n_of_a_kind = cards
        .iter()
        .map(|&card| {
            let n_this_card = cards
                .iter()
                .filter(|&other_card| card == *other_card)
                .count();
            n_this_card
        })
        .max()
        .expect("hand not empty");
    let number_unique_cards = cards
        .iter()
        .enumerate()
        .filter(|(card_id, &card)| {
            let card_is_unique = !cards
                .iter()
                .enumerate()
                .any(|(other_card_id, &other_card)| {
                    *card_id != other_card_id && card == other_card
                });
            card_is_unique
        })
        .count();

    match (n_of_a_kind, number_unique_cards) {
        (5, _) => HandType::FiveOfAKind,
        (4, _) => HandType::FourOfAKind,
        (3, 0) => HandType::FullHouse,
        (3, _) => HandType::ThreeOfAKind,
        (2, 1) => HandType::TwoPair,
        (2, 3) => HandType::OnePair,
        (_, 5) => HandType::HighCard,
        _ => unreachable!(),
    }
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, c) = alt((
        map(tag("2"), |_| Card::Two),
        map(tag("3"), |_| Card::Three),
        map(tag("4"), |_| Card::Four),
        map(tag("5"), |_| Card::Five),
        map(tag("6"), |_| Card::Six),
        map(tag("7"), |_| Card::Seven),
        map(tag("8"), |_| Card::Eight),
        map(tag("9"), |_| Card::Nine),
        map(tag("T"), |_| Card::Ten),
        map(tag("J"), |_| Card::Jack),
        map(tag("Q"), |_| Card::Queen),
        map(tag("K"), |_| Card::King),
        map(tag("A"), |_| Card::Ace),
    ))(input)?;
    Ok((input, c))
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    struct CardWrapper {
        cards: [Card; 5],
        i: usize,
    }
    impl CardWrapper {
        fn add_card(&mut self, card: Card) {
            self.cards[self.i] = card;
            self.i += 1;
        }

        fn new() -> CardWrapper {
            CardWrapper {
                cards: [Card::Ace; 5],
                i: 0,
            }
        }
    }

    let (input, card_wrapper) = fold_many1(
        parse_card,
        CardWrapper::new,
        |mut acc: CardWrapper, card| {
            acc.add_card(card);
            acc
        },
    )(input)?;
    Ok((input, Hand::new(card_wrapper.cards)))
}

fn parse_hand_tuples(input: &str) -> IResult<&str, Vec<HandTuple>> {
    let parse_u32 = map_res(digit1, |s: &str| s.parse::<u32>());
    let parse_line = separated_pair(parse_hand, space1, parse_u32);
    let parse_hand_tuple = map(parse_line, |(hand, bid)| HandTuple { hand, bid });
    separated_list1(newline, parse_hand_tuple)(input)
}

#[cfg(test)]
#[test]
fn example() {
    let s = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    assert_eq!(process(s), 6440);
}

#[test]
fn part_1() {
    let s = include_str!("../input.txt");
    assert_eq!(process(s), 251121738);
}
