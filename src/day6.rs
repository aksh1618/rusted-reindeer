#[derive(Debug)]
pub struct RaceRecord {
    time: u32,
    distance: u32,
}

impl RaceRecord {
    fn record_broken(&self, time_held: u32) -> bool {
        (self.time - time_held) * time_held > self.distance
    }
}

#[allow(clippy::unwrap_used)]
pub fn generator(input: &str) -> Vec<RaceRecord> {
    let mut lines: Vec<&str> = input.lines().collect();
    let times = lines.remove(0);
    let distances = lines.remove(0);
    std::iter::zip(
        times
            .strip_prefix("Time:")
            .unwrap()
            .split_ascii_whitespace()
            .map(|num| num.parse::<u32>().unwrap()),
        distances
            .strip_prefix("Distance:")
            .unwrap()
            .split_ascii_whitespace()
            .map(|num| num.parse::<u32>().unwrap()),
    )
    .map(|(time, distance)| RaceRecord { time, distance })
    .collect()
}

pub fn part1(races: &[RaceRecord]) -> u32 {
    races.iter().map(winning_ways).product()
}

fn winning_ways(race_record: &RaceRecord) -> u32 {
    let min_holding_time = (0..=race_record.time)
        .find(|time_held| race_record.record_broken(*time_held))
        .unwrap_or(0);
    let max_holding_time = (0..=race_record.time)
        .rfind(|time_held| race_record.record_broken(*time_held))
        .unwrap_or(0);
    if min_holding_time == 0 && max_holding_time == 0 {
        0
    } else {
        max_holding_time - min_holding_time + 1
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use indoc::indoc;

    const EXAMPLE_INPUT: &str = indoc! {"
        Time:      7  15   30
        Distance:  9  40  200
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&generator(EXAMPLE_INPUT)), 288);
    }
}
