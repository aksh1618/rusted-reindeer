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

pub fn part2(input: &str) -> i32 {
    todo!()
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

    #[test]
    fn part2_example() {
        assert_eq!(part2("test"), 4);
    }
}
