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
        mappings.push(mapping)
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

#[cfg(test)]
mod tests {

    use super::*;
    use indoc::indoc;
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
    }
}
