use std::collections::{HashMap, HashSet};

type Pile = HashMap<u32, Card>;

pub struct Card {
    winning_numbers: HashSet<u32>,
    player_numbers: HashSet<u32>,
}

impl Card {
    fn get_won_numbers_count(&self) -> u32 {
        self.winning_numbers
            .iter()
            .filter(|num| self.player_numbers.contains(num))
            .count() as u32
    }
}

pub fn generator(input: &str) -> Pile {
    let expectation = "Input should be in prescribed format";
    input
        .lines()
        .map(|line| {
            let (card_string, numbers) = line.split_once(':').expect(expectation);
            let (_, card_num) = card_string.split_once(' ').expect(expectation);
            let card_num = card_num.trim().parse().expect(expectation);
            let (winning_numbers, player_numbers) = numbers.split_once('|').expect(expectation);
            let winning_numbers = winning_numbers
                .split_whitespace()
                .map(|num| num.parse().expect(expectation))
                .collect();
            let player_numbers = player_numbers
                .split_whitespace()
                .map(|num| num.parse().expect(expectation))
                .collect();
            (
                card_num,
                Card {
                    winning_numbers,
                    player_numbers,
                },
            )
        })
        .collect()
}

pub fn part1(pile: &Pile) -> u32 {
    pile.values()
        .map(|card| {
            let won_numbers_count = card.get_won_numbers_count();
            if won_numbers_count > 0 {
                u32::pow(2, won_numbers_count - 1)
            } else {
                0
            }
        })
        .sum()
}

pub fn part2(pile: &Pile) -> u32 {
    let mut new_pile_counts = HashMap::new();
    for i in 1..=pile.len() as u32 {
        new_pile_counts.insert(i, 1);
    }
    for i in 1..=pile.len() as u32 {
        let card = pile.get(&i).unwrap();
        let won_numbers_count = card.get_won_numbers_count();
        let self_count = new_pile_counts.get(&i).unwrap().to_owned();
        for j in 1..=won_numbers_count {
            new_pile_counts
                .entry(i + j)
                .and_modify(|cnt| *cnt += self_count)
                .or_insert(0);
        }
    }
    new_pile_counts.values().sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn part1_example() {
        let input = indoc! {"
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "};
        assert_eq!(part1(&generator(input)), 13);
    }

    #[test]
    fn part2_example() {
        let input = indoc! {"
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "};
        assert_eq!(part2(&generator(input)), 30);
    }
}
