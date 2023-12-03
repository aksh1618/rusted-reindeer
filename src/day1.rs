fn extract_calibration_value(sentence: &str) -> u32 {
    let first_digit = sentence
        .chars()
        .find(|c| c.is_ascii_digit())
        .and_then(|c| c.to_digit(10))
        .expect("There should be a digit in the sentence");
    let last_digit = sentence
        .chars()
        .rfind(|c| c.is_ascii_digit())
        .and_then(|c| c.to_digit(10))
        .expect("There should be a digit in the sentence");
    (first_digit * 10) + last_digit
}

pub fn part1(input: &str) -> u32 {
    input
        .split_terminator('\n')
        // .filter(|s| !s.is_empty())
        .map(extract_calibration_value)
        .sum()
}

const DIGIT_WORDS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

// const DIGIT_WORDS_REVERSED: [&str; 10] = [
//     "orez", "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin",
// ];

const DIGIT_CHARS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

fn extract_calibration_value_strings_take1_fixed(sentence: &str) -> u32 {
    let mut first_index = sentence.len();
    let mut last_index = 0;
    let mut first_num = 0;
    let mut last_num = 0;
    for i in 1..=9 {
        sentence
            .match_indices(DIGIT_WORDS[i]) // Disjoint, but works as no digit word contains its starting characters
            .for_each(|(index, _)| {
                if index <= first_index {
                    first_index = index;
                    first_num = i;
                }
                if index >= last_index {
                    last_index = index;
                    last_num = i;
                }
            });
        sentence
            .match_indices(DIGIT_CHARS[i])
            .for_each(|(index, _)| {
                if index <= first_index {
                    first_index = index;
                    first_num = i;
                }
                if index >= last_index {
                    last_index = index;
                    last_num = i;
                }
            });
    }
    let calibration_value = 10 * first_num + last_num;
    // println!("{}: {}", sentence, calibration_value);
    calibration_value as u32
}

pub fn part2_take1(input: &str) -> u32 {
    input
        .split_terminator('\n')
        .map(extract_calibration_value_strings_take1_fixed)
        .sum()
}

fn extract_calibration_value_strings_take2(sentence: &str) -> u32 {
    let first_digit = sentence
        .chars()
        .enumerate()
        .find_map(|(i, c)| {
            c.to_digit(10)
                .filter(|i| *i != 0)
                .or_else(|| starts_with_digit_word(sentence, i))
        })
        .expect("There should be a digit in the sentence");
    let len = sentence.len();
    let last_digit = sentence
        .chars()
        .rev()
        .enumerate()
        .find_map(|(i, c)| {
            c.to_digit(10)
                .filter(|i| *i != 0)
                .or_else(|| ends_with_digit_word(sentence, len - i - 1))
        })
        .expect("There should be a digit in the sentence");
    // let calibration_value = 10 * first_digit + last_digit;
    // println!("{}: {}", sentence, calibration_value);
    // calibration_value
    (first_digit * 10) + last_digit
}

fn starts_with_digit_word(sentence: &str, index: usize) -> Option<u32> {
    let substring = sentence
        .get(index..sentence.len())
        .expect("string should be single length characters");
    DIGIT_WORDS
        .iter()
        .enumerate()
        .skip(1)
        .find(|(_, &dw)| substring.starts_with(dw))
        .map(|(num, _)| num as u32)
}

fn ends_with_digit_word(sentence: &str, index: usize) -> Option<u32> {
    let substring = sentence
        .get(0..=index)
        .expect("string should be single length characters");
    DIGIT_WORDS
        .iter()
        .enumerate()
        .skip(1)
        .find(|(_, &dw)| substring.ends_with(dw))
        .map(|(num, _)| num as u32)
}

/// Went for take_2 as take_1 wasn't working as I was looking for find_indices.
/// Later found match_indices, but still take_2 is ~10x faster than take_1
pub fn part2_take2(input: &str) -> u32 {
    input
        .split_terminator('\n')
        .map(extract_calibration_value_strings_take2)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn part1_example() {
        let input = indoc! {"
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        "};
        assert_eq!(part1(input), 142);
    }

    fn part2(input: &str) -> u32 {
        // part2_take1(input)
        part2_take2(input)
    }

    #[test]
    fn part2_example() {
        let input = indoc! {"
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        "};
        assert_eq!(part2(input), 281);
    }

    #[test]
    fn part2_single_digit_case() {
        assert_eq!(part2("treb7uchet"), 77);
        assert_eq!(part2("4l"), 44);
        assert_eq!(part2("e9"), 99);
        assert_eq!(part2("1"), 11);
        assert_eq!(part2("two"), 22);
    }

    #[test]
    fn part2_tricky_case() {
        assert_eq!(part2("2twone"), 21);
    }

    #[test]
    fn part2_ending_chars_case() {
        assert_eq!(part2("2three1"), 21);
        assert_eq!(part2("42one35"), 45);
    }

    #[test]
    fn part2_repeating_digit_case() {
        assert_eq!(part2("2three2"), 22);
        assert_eq!(part2("four123four"), 44);
    }

    #[test]
    fn part2_zero_case() {
        assert_eq!(part2("012zero"), 12);
        assert_eq!(part2("zero120"), 12);
    }

}
