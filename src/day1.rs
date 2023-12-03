pub fn generator(input: &str) -> &str {
    input
}

pub fn part1(input: &str) -> i32 {
    input.len().try_into().unwrap()
}

pub fn part2(input: &str) -> i32 {
    input.len().try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(generator("test")), 4);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(generator("test")), 4);
    }
}
