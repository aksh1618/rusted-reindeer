type History = Vec<i32>;
type Sequence = Vec<i32>;

#[allow(clippy::unwrap_used)]
pub fn generator(input: &str) -> Vec<History> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num_str| num_str.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

pub fn create_sequences(history: &History) -> Vec<Sequence> {
    let mut sequences = vec![history.to_owned()];
    loop {
        let diff_sequence = sequences
            .last()
            .expect("sequences should not be empty")
            .windows(2)
            .map(|nums| nums[1] - nums[0])
            .collect::<Vec<_>>();
        if diff_sequence.iter().all(|num| *num == 0) {
            break sequences.push(diff_sequence);
        }
        sequences.push(diff_sequence);
    }
    sequences
}

pub fn part1(report: &[History]) -> i32 {
    report
        .iter()
        .map(create_sequences)
        .map(|seqs| {
            seqs.iter().rev().fold(0, |num, seq| {
                *seq.last().expect("Sequence should be non-empty") + num
            })
        })
        .sum()
}

pub fn part2(report: &[History]) -> i32 {
    report
        .iter()
        .map(create_sequences)
        .map(|seqs| {
            seqs.iter().rev().fold(0, |num, seq| {
                *seq.first().expect("Sequence should be non-empty") - num
            })
        })
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;
    use indoc::indoc;

    const EXAMPLE_INPUT: &str = indoc! {"
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45
    "};

    #[test]
    fn part1_example() {
        // println!("{:?}", generator(EXAMPLE_INPUT));
        assert_eq!(part1(&generator(EXAMPLE_INPUT)), 114);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&generator(EXAMPLE_INPUT)), 2);
    }
}
