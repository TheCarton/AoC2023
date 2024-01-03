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
    let n_unique = cards
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
    let hand_type = if n_of_a_kind == 5 {
        assert!(n_unique == 0);
        HandType::FiveOfAKind
    } else if n_of_a_kind == 4 {
        assert!(n_unique == 1);
        HandType::FourOfAKind
    } else if n_of_a_kind == 3 && n_unique == 0 {
        HandType::FullHouse
    } else if n_of_a_kind == 3 {
        assert!(n_unique <= 3);
        HandType::ThreeOfAKind
    } else if n_unique == 1 {
        HandType::TwoPair
    } else if n_unique == 3 {
        HandType::OnePair
    } else {
        assert!(n_unique == 5);
        HandType::HighCard
    };
    hand_type
}
// 32T3K
fn parse_card(input: &str) -> IResult<&str, Card> {
    let parse_two = map(tag("2"), |_| Card::Two);
    let parse_three = map(tag("3"), |_| Card::Three);
    let parse_four = map(tag("4"), |_| Card::Four);
    let parse_five = map(tag("5"), |_| Card::Five);
    let parse_six = map(tag("6"), |_| Card::Six);
    let parse_seven = map(tag("7"), |_| Card::Seven);
    let parse_eight = map(tag("8"), |_| Card::Eight);
    let parse_nine = map(tag("9"), |_| Card::Nine);
    let parse_ten = map(tag("T"), |_| Card::Ten);
    let parse_jack = map(tag("J"), |_| Card::Jack);
    let parse_queen = map(tag("Q"), |_| Card::Queen);
    let parse_king = map(tag("K"), |_| Card::King);
    let parse_ace = map(tag("A"), |_| Card::Ace);
    let (input, c) = alt((
        parse_two,
        parse_three,
        parse_four,
        parse_five,
        parse_six,
        parse_seven,
        parse_eight,
        parse_nine,
        parse_ten,
        parse_jack,
        parse_queen,
        parse_king,
        parse_ace,
    ))(input)?;
    Ok((input, c))
}

// 32T3K
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
