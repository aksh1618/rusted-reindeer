use std::ops::RangeInclusive;

type Seeds = Vec<u32>;
type Mapping = Vec<Vec<u32>>;

#[derive(Debug, PartialEq)]
pub struct GardenAlmanac {
    seed_to_soil: Mapping,
    soil_to_fertilizer: Mapping,
    fertilizer_to_water: Mapping,
    water_to_light: Mapping,
    light_to_temperature: Mapping,
    temperature_to_humidity: Mapping,
    humidity_to_location: Mapping,
}

impl GardenAlmanac {
    #[allow(clippy::trivially_copy_pass_by_ref)] // && required to allow use as function reference
    fn apply_mapping(input: u32, mapping: &&Mapping) -> u32 {
        mapping
            .iter()
            .find(|range_mapping| {
                let input_start = range_mapping[1];
                let input_range = input_start..input_start + range_mapping[2];
                input_range.contains(&input)
            })
            .map(|matching_range_mapping| {
                input + matching_range_mapping[0] - matching_range_mapping[1]
            })
            // .unwrap_or_else(|| panic!("There should be at least one matching range for {} in {:?}", input, mapping))
            .unwrap_or(input)
    }

    /// Takes a range and applies the provided mapping to return a set of output ranges
    ///
    /// A vizualization: (`|` indicates start & end of a range)
    /// ```text
    /// Number Line     ->    |----------------------------------|
    ///                      22                                  57
    /// Input range     ->       |^^^^^^^^^^i^^^^^^^^^^|
    /// Input subranges ->       |~a~||~~b~~||~c||d||~e|
    /// Mapping source  ->   ..._____||##x##||#y||_||#z##||______...
    /// Mapping dest    ->   ...___||**x**||____||*z**||_||*y||__...
    /// Output ranges   -> a]    |+++|
    ///                    b]       |++x++|
    ///                    c]                             |+y|
    ///                    d]                    |+|
    ///                    e]                    |+z|
    /// ```
    /// The above example will be written as follows:
    /// ```rust
    /// assert_eq!(
    ///     GardenAlmanac::apply_mapping_to_range(
    ///         &(25..=47), // i
    ///         &vec![
    ///             vec![28, 30, 7], // x
    ///             vec![50, 37, 4], // y
    ///             vec![41, 44, 6], // z
    ///         ]
    ///     )
    ///     .iter()
    ///     .collect::<HashSet<_>>(),
    ///     [
    ///         25..=29, // a
    ///         28..=34, // b
    ///         50..=53, // c
    ///         41..=43, // d
    ///         41..=44, // e
    ///     ]
    ///     .iter()
    ///     .collect::<HashSet<_>>()
    /// );
    /// ```
    #[allow(clippy::range_minus_one)] // Using inclusive ranges here makes the code simpler
    fn apply_mapping_to_range(
        input_range: &RangeInclusive<u32>,
        mapping: &Mapping,
    ) -> Vec<RangeInclusive<u32>> {
        let mut mapped_input_subranges = Vec::new();
        let mut output_ranges = Vec::new();
        for range_mapping in mapping {
            let source_start = range_mapping[1];
            let source_range = source_start..=(source_start + range_mapping[2] - 1);
            if source_range.end() < input_range.start() || source_range.start() > input_range.end()
            {
                continue;
            }
            let input_subrange_start = u32::max(*input_range.start(), *source_range.start());
            let input_subrange_end = u32::min(*source_range.end(), *input_range.end());
            let destination_start = range_mapping[0];
            let destination_subrange_start =
                input_subrange_start + destination_start - source_start;
            let destination_subrange_end = input_subrange_end + destination_start - source_start;
            let input_subrange = input_subrange_start..=input_subrange_end;
            let destination_subrange = destination_subrange_start..=destination_subrange_end;
            mapped_input_subranges.push(input_subrange);
            output_ranges.push(destination_subrange);
        }
        let unmapped_input_ranges =
            get_not_covered_ranges(input_range, &mut mapped_input_subranges);
        output_ranges.extend(unmapped_input_ranges);
        output_ranges
    }

    fn get_location_for_seed(&self, seed: u32) -> u32 {
        [
            &self.seed_to_soil,
            &self.soil_to_fertilizer,
            &self.fertilizer_to_water,
            &self.water_to_light,
            &self.light_to_temperature,
            &self.temperature_to_humidity,
            &self.humidity_to_location,
        ]
        .iter()
        .fold(seed, GardenAlmanac::apply_mapping)
    }

    fn get_location_ranges_for_seed_range(
        &self,
        seed_range: RangeInclusive<u32>,
    ) -> Vec<RangeInclusive<u32>> {
        [
            &self.seed_to_soil,
            &self.soil_to_fertilizer,
            &self.fertilizer_to_water,
            &self.water_to_light,
            &self.light_to_temperature,
            &self.temperature_to_humidity,
            &self.humidity_to_location,
        ]
        .iter()
        .fold(vec![seed_range], |ranges, mapping| {
            ranges
                .iter()
                .flat_map(|range| GardenAlmanac::apply_mapping_to_range(range, mapping))
                .collect()
        })
    }
}

/// Takes a range and disjoint subranges to return subranges that were not covered by the subranges
///
/// A vizualization: (`|` indicates start & end of a range)
/// ```text
/// Input range             ->  |*********************|
/// Covered subranges       ->  |###|-------|##||#|----
/// Not covered subranges   ->       |#####|       |##|
/// ```
#[allow(clippy::range_minus_one)] // Using inclusive ranges here makes the code simpler
fn get_not_covered_ranges(
    range: &RangeInclusive<u32>,
    covered_subranges: &mut [RangeInclusive<u32>],
) -> Vec<RangeInclusive<u32>> {
    // covered_subranges.sort_by(|this, other| this.start().cmp(other.start()));
    covered_subranges.sort_unstable_by_key(|range| *range.start());
    let mut not_covered_ranges = Vec::new();
    let mut iter = covered_subranges.iter();
    let Some(first_subrange) = iter.next() else {
        return vec![range.clone()];
    };
    if first_subrange.start() > range.start() {
        not_covered_ranges.push(*range.start()..=(first_subrange.start() - 1));
    }
    let mut previous_subrange = first_subrange;
    for cur_subrange in iter.as_ref() {
        if *cur_subrange.start() > (previous_subrange.end() + 1) {
            not_covered_ranges.push((previous_subrange.end() + 1)..=(cur_subrange.start() - 1));
        }
        previous_subrange = cur_subrange;
    }
    if previous_subrange.end() < range.end() {
        not_covered_ranges.push((first_subrange.end() + 1)..=*range.end());
    }
    not_covered_ranges
}

pub fn generator(input: &str) -> (Seeds, GardenAlmanac) {
    let expectation = "Input should be in prescribed format";
    let mut lines = input.lines();
    let seeds = lines
        .next()
        .expect(expectation)
        .split_once(':')
        .expect(expectation)
        .1
        .split_whitespace()
        .map(|num_str| num_str.parse::<u32>().expect(expectation))
        .collect();
    lines.next();
    let mut mappings = Vec::new();
    for _ in 0..7 {
        lines.next();
        let mut mapping = Vec::new();
        for map_line in lines.by_ref() {
            if map_line.trim().is_empty() {
                break;
            }
            let partial_mapping = map_line
                .split_whitespace()
                .map(|num_str| num_str.parse::<u32>().expect(expectation))
                .collect::<Vec<u32>>();
            mapping.push(partial_mapping);
        }
        mappings.push(mapping);
    }
    (
        seeds,
        GardenAlmanac {
            seed_to_soil: mappings.remove(0),
            soil_to_fertilizer: mappings.remove(0),
            fertilizer_to_water: mappings.remove(0),
            water_to_light: mappings.remove(0),
            light_to_temperature: mappings.remove(0),
            temperature_to_humidity: mappings.remove(0),
            humidity_to_location: mappings.remove(0),
        },
    )
}

pub fn part1((seeds, almanac): &(Seeds, GardenAlmanac)) -> u32 {
    seeds
        .iter()
        .map(|seed| almanac.get_location_for_seed(*seed))
        .min()
        .expect("There should be atleast one seed, and every seed should have at least one location mapping")
}

#[allow(dead_code)]
pub fn part2_naive((seeds, almanac): &(Seeds, GardenAlmanac)) -> u32 {
    std::iter::zip(
        seeds.iter().enumerate().filter(|(i, _)| i % 2 == 0).map(|(_, seed)| seed),
        seeds.iter().enumerate().filter(|(i, _)| i % 2 != 0).map(|(_, seed)| seed)
    ).filter_map(|(seed_start, seed_count)| {
        (*seed_start..*seed_start+*seed_count)
        .map(|seed| almanac.get_location_for_seed(seed))
        .min()
    })
        .min()
        .expect("There should be atleast one seed, and every seed should have at least one location mapping")
}

#[allow(clippy::range_minus_one)] // Using inclusive ranges here makes the code simpler
pub fn part2((seeds, almanac): &(Seeds, GardenAlmanac)) -> u32 {
    std::iter::zip(
        seeds.iter().enumerate().filter(|(i, _)| i % 2 == 0).map(|(_, seed)| seed),
        seeds.iter().enumerate().filter(|(i, _)| i % 2 != 0).map(|(_, seed)| seed),
    )
        .flat_map(|(seed_start, seed_count)| {
            almanac.get_location_ranges_for_seed_range(*seed_start..=*seed_start+*seed_count-1)
        })
        .min_by(|this_range, other_range| this_range.start().cmp(other_range.start()))
        .map(|min_start_range| *min_start_range.start())
        .expect("There should be atleast one seed, and every seed should have at least one location mapping")
}

#[allow(clippy::range_minus_one)] // Using inclusive ranges here makes the code simpler
pub fn part1_with_ranges((seeds, almanac): &(Seeds, GardenAlmanac)) -> u32 {
    seeds.iter()
        .flat_map(|seed_start| {
            almanac.get_location_ranges_for_seed_range(*seed_start..=*seed_start)
        })
        .min_by(|this_range, other_range| this_range.start().cmp(other_range.start()))
        .map(|min_start_range| *min_start_range.start())
        .expect("There should be atleast one seed, and every seed should have at least one location mapping")
}

#[cfg(test)]
mod tests {

    use super::*;
    use indoc::indoc;
    use std::collections::HashSet;

    const EXAMPLE_INPUT: &str = indoc! {"
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4
        "};

    #[test]
    fn generator_example() {
        // println!("{:?}", generator(EXAMPLE_INPUT));
        let mut expected_mappings = vec![
            vec![vec![50, 98, 2], vec![52, 50, 48]],
            vec![vec![0, 15, 37], vec![37, 52, 2], vec![39, 0, 15]],
            vec![
                vec![49, 53, 8],
                vec![0, 11, 42],
                vec![42, 0, 7],
                vec![57, 7, 4],
            ],
            vec![vec![88, 18, 7], vec![18, 25, 70]],
            vec![vec![45, 77, 23], vec![81, 45, 19], vec![68, 64, 13]],
            vec![vec![0, 69, 1], vec![1, 0, 69]],
            vec![vec![60, 56, 37], vec![56, 93, 4]],
        ];
        let expected = (
            vec![79, 14, 55, 13],
            GardenAlmanac {
                seed_to_soil: expected_mappings.remove(0),
                soil_to_fertilizer: expected_mappings.remove(0),
                fertilizer_to_water: expected_mappings.remove(0),
                water_to_light: expected_mappings.remove(0),
                light_to_temperature: expected_mappings.remove(0),
                temperature_to_humidity: expected_mappings.remove(0),
                humidity_to_location: expected_mappings.remove(0),
            },
        );
        assert_eq!(generator(EXAMPLE_INPUT), expected);
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&generator(EXAMPLE_INPUT)), 35);
        assert_eq!(part1_with_ranges(&generator(EXAMPLE_INPUT)), 35);
    }

    #[test]
    fn apply_mapping_to_range_doc_example() {
        assert_eq!(
            GardenAlmanac::apply_mapping_to_range(
                &(25..=47), // i
                &vec![
                    vec![28, 30, 7], // x
                    vec![50, 37, 4], // y
                    vec![41, 44, 6], // z
                ]
            )
            .iter()
            .collect::<HashSet<_>>(),
            [
                25..=29, // a
                28..=34, // b
                50..=53, // c
                41..=43, // d
                41..=44, // e
            ]
            .iter()
            .collect::<HashSet<_>>()
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2_naive(&generator(EXAMPLE_INPUT)), 46);
        assert_eq!(part2(&generator(EXAMPLE_INPUT)), 46);
    }
}
