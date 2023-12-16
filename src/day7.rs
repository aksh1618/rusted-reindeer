use itertools::Itertools;
use std::collections::HashSet;

// const Cards: [char; 13] = [
//     'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
// ];

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

type HandCards = [Card; 5];

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Hand {
    hand_type: HandType,
    cards: HandCards,
}

impl Hand {
    fn from_card_str(cards: &str) -> Self {
        assert!(cards.len() == 5);
        if let Some(cards) = cards
            .chars()
            .map(|char| match char {
                'A' => Card::Ace,
                'K' => Card::King,
                'Q' => Card::Queen,
                'J' => Card::Jack,
                'T' => Card::Ten,
                '9' => Card::Nine,
                '8' => Card::Eight,
                '7' => Card::Seven,
                '6' => Card::Six,
                '5' => Card::Five,
                '4' => Card::Four,
                '3' => Card::Three,
                '2' => Card::Two,
                _ => unreachable!(),
            })
            .collect_tuple::<(Card, Card, Card, Card, Card)>()
        {
            Hand::from_cards([cards.0, cards.1, cards.2, cards.3, cards.4])
        } else {
            unreachable!()
        }
    }

    fn from_cards(cards: HandCards) -> Self {
        let counts = cards.iter().counts();
        let distinct_cards = counts.len();
        let count_values = counts.values().collect::<HashSet<_>>();
        let hand_type = match distinct_cards {
            1 => HandType::FiveOfAKind,
            2 if count_values.contains(&4) => HandType::FourOfAKind,
            2 if count_values.contains(&3) => HandType::FullHouse,
            3 if count_values.contains(&3) => HandType::ThreeOfAKind,
            3 if count_values.contains(&2) => HandType::TwoPair,
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => unreachable!(),
        };
        Hand { hand_type, cards }
    }
}

#[allow(clippy::unwrap_used)]
pub fn generator(input: &str) -> Vec<(Hand, u32)> {
    input
        .lines()
        .map(|line| {
            let (cards_str, bid) = line.split_once(' ').unwrap();
            (Hand::from_card_str(cards_str), bid.parse::<u32>().unwrap())
        })
        .collect()
}

pub fn part1(hand_and_bid_list: &[(Hand, u32)]) -> u32 {
    hand_and_bid_list
        .iter()
        .sorted_by(|(hand_a, _), (hand_b, _)| Hand::cmp(hand_a, hand_b))
        .enumerate()
        .inspect(|(i, (hand, bid))| println!("{} {hand:?} {bid}", i + 1))
        .map(|(i, (_, bid))| bid * (i as u32 + 1))
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;
    use indoc::indoc;

    const EXAMPLE_INPUT: &str = indoc! {"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
    "};

    #[test]
    fn hand_ordering() {
        let five_of_a_kind = Hand::from_card_str("AAAAA");
        let four_of_a_kind = Hand::from_card_str("33332");
        let four_of_a_kind_other = Hand::from_card_str("2AAAA");
        let three_of_a_kind = Hand::from_card_str("T55J5");
        let two_pair = Hand::from_card_str("KTJJT");
        assert!(five_of_a_kind > four_of_a_kind);
        assert!(five_of_a_kind > four_of_a_kind_other);
        assert!(four_of_a_kind > four_of_a_kind_other);
        assert!(three_of_a_kind > two_pair);
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&generator(EXAMPLE_INPUT)), 6440);
    }
}
