#[derive(Debug)]
pub struct Races {
    race_records: Vec<RaceRecord>,
    actual_race_record: RaceRecord,
}

#[derive(Debug)]
pub struct RaceRecord {
    time: u64,
    distance: u64,
}

impl RaceRecord {
    fn record_broken(&self, time_held: u64) -> bool {
        (self.time - time_held) * time_held > self.distance
    }
}

#[allow(clippy::unwrap_used)]
pub fn generator(input: &str) -> Races {
    let mut lines: Vec<&str> = input.lines().collect();
    let times = lines.remove(0).strip_prefix("Time:").unwrap();
    let distances = lines.remove(0).strip_prefix("Distance:").unwrap();
    let race_records = std::iter::zip(
        times
            .split_ascii_whitespace()
            .map(|num| num.parse::<u64>().unwrap()),
        distances
            .split_ascii_whitespace()
            .map(|num| num.parse::<u64>().unwrap()),
    )
    .map(|(time, distance)| RaceRecord { time, distance })
    .collect();
    let actual_race_record = RaceRecord {
        time: times
            .split_ascii_whitespace()
            .collect::<String>()
            .parse::<u64>()
            .unwrap(),
        distance: distances
            .split_ascii_whitespace()
            .collect::<String>()
            .parse::<u64>()
            .unwrap(),
    };
    Races {
        race_records,
        actual_race_record,
    }
}

pub fn part1(races: &Races) -> u64 {
    races.race_records.iter().map(winning_ways).product()
}

pub fn part2(races: &Races) -> u64 {
    winning_ways(&races.actual_race_record)
}

fn winning_ways(race_record: &RaceRecord) -> u64 {
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

    #[test]
    fn part2_example() {
        assert_eq!(part2(&generator(EXAMPLE_INPUT)), 71503);
    }
}
