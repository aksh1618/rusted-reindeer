use itertools::Itertools;
use std::collections::HashSet;

// const Cards: [char; 13] = [
//     'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
// ];

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Card {
    Joker,
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

#[derive(Default)]
struct CardsConfig {
    j_is_joker: bool,
}

impl Hand {
    fn from_card_str(cards: &str, config: &CardsConfig) -> Self {
        assert!(cards.len() == 5);
        if let Some(cards) = cards
            .chars()
            .map(|char| match char {
                'A' => Card::Ace,
                'K' => Card::King,
                'Q' => Card::Queen,
                'J' => {
                    if config.j_is_joker {
                        Card::Joker
                    } else {
                        Card::Jack
                    }
                }
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
        if let Some(joker_count) = counts.get(&Card::Joker) {
            let hand_type = hand_type_with_jokers(&hand_type, *joker_count);
            Hand { hand_type, cards }
        } else {
            Hand { hand_type, cards }
        }
    }
}

#[allow(clippy::match_same_arms)]
fn hand_type_with_jokers(old_hand_type: &HandType, joker_count: usize) -> HandType {
    match old_hand_type {
        HandType::HighCard => match joker_count {
            1 => HandType::OnePair, // Include any other card
            _ => unreachable!(),    // At max 1 of each card
        },
        HandType::OnePair => match joker_count {
            1 => HandType::ThreeOfAKind, // Include the existing pair
            2 => HandType::ThreeOfAKind, // Joker pair => Include any other card
            _ => unreachable!(),         // At max 2 of each card
        },
        HandType::TwoPair => match joker_count {
            1 => HandType::FullHouse, // Include one of the existing pairs to get triplet & pair
            2 => HandType::FourOfAKind, // Joker pair => Include other pair
            _ => unreachable!(),      // At max 2 of each card
        },
        HandType::ThreeOfAKind => match joker_count {
            1 => HandType::FourOfAKind, // Include the existing triplet
            3 => HandType::FourOfAKind, // Joker triplet => Include any other card
            _ => unreachable!(),        // At max 3 of each card & Joker pair will make FullHouse
        },
        HandType::FullHouse => match joker_count {
            2 => HandType::FiveOfAKind, // Joker pair => Include the triplet
            3 => HandType::FiveOfAKind, // Joker triplet => Include the pair
            _ => unreachable!(),        // At max 3 of each card, and 1 Joker will make ThreeOfAKind
        },
        HandType::FourOfAKind => match joker_count {
            1 => HandType::FiveOfAKind, // Include the Quadruplet
            4 => HandType::FiveOfAKind, // Joker Quadruplet => Include the fifth card
            _ => unreachable!(), // At max 4 of each card, and 2/3 Jokers would no longer be FourOfAKind
        },
        HandType::FiveOfAKind => HandType::FiveOfAKind, // What more you want?
    }
}

#[allow(clippy::unwrap_used)]
fn parse_hands(input: &str, cards_config: &CardsConfig) -> Vec<(Hand, u32)> {
    input
        .lines()
        .map(|line| {
            let (cards_str, bid) = line.split_once(' ').unwrap();
            (
                Hand::from_card_str(cards_str, cards_config),
                bid.parse::<u32>().unwrap(),
            )
        })
        .collect()
}

fn calculate_bid(input: &str, cards_config: &CardsConfig) -> u32 {
    parse_hands(input, cards_config)
        .iter()
        .sorted_by(|(hand_a, _), (hand_b, _)| Hand::cmp(hand_a, hand_b))
        .enumerate()
        // .inspect(|(i, (hand, bid))| println!("{} {hand:?} {bid}", i + 1))
        .map(|(i, (_, bid))| bid * (i as u32 + 1))
        .sum()
}

pub fn part1(input: &str) -> u32 {
    calculate_bid(input, &CardsConfig::default())
}

pub fn part2(input: &str) -> u32 {
    calculate_bid(input, &CardsConfig { j_is_joker: true })
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
        let five_of_a_kind = Hand::from_card_str("AAAAA", &CardsConfig::default());
        let four_of_a_kind = Hand::from_card_str("33332", &CardsConfig::default());
        let four_of_a_kind_other = Hand::from_card_str("2AAAA", &CardsConfig::default());
        let three_of_a_kind = Hand::from_card_str("T55J5", &CardsConfig::default());
        let two_pair = Hand::from_card_str("KTJJT", &CardsConfig::default());
        assert!(five_of_a_kind > four_of_a_kind);
        assert!(five_of_a_kind > four_of_a_kind_other);
        assert!(four_of_a_kind > four_of_a_kind_other);
        assert!(three_of_a_kind > two_pair);
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 6440);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 5905);
    }
}
