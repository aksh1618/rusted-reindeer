use std::{
    collections::{HashMap, HashSet},
    fmt,
};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Coordinate(u32, u32);
pub struct EngineSchematic {
    max_coords: Coordinate,
    items: HashMap<Coordinate, SchematicItem>,
}

impl EngineSchematic {
    fn get_item(&self, x: u32, y: u32) -> Option<&SchematicItem> {
        self.items.get(&Coordinate(x, y))
    }
    fn is_digit(&self, coordinate: &Coordinate) -> bool {
        self.items
            .get(coordinate)
            .filter(|item| matches!(item, SchematicItem::Digit(_)))
            .is_some()
    }
}

#[derive(Debug, PartialEq)]
pub enum SchematicItem {
    Symbol(char),
    Digit(u32),
}

impl fmt::Display for EngineSchematic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Coordinate(max_x, max_y) = self.max_coords;
        let mut output = String::new();
        for y in 0..=max_y {
            for x in 0..=max_x {
                if let Some(schematic_item) = self.items.get(&Coordinate(x, y)) {
                    match schematic_item {
                        SchematicItem::Symbol(sym) => output.push(*sym),
                        SchematicItem::Digit(num) => output.push_str(num.to_string().as_str()),
                    }
                } else {
                    output.push('.');
                }
            }
            output.push('\n');
        }
        write!(f, "{}", output)
    }
}

pub fn generator(input: &str) -> EngineSchematic {
    let mut engine_schematic_items = HashMap::new();
    let mut x_max = 0;
    let mut y_max = 0;
    input.lines().enumerate().for_each(|(y, schematic_line)| {
        schematic_line
            .chars()
            .enumerate()
            .for_each(|(x, c)| match c {
                '.' => (),
                '0'..='9' => {
                    engine_schematic_items.insert(
                        Coordinate(x as u32, y as u32),
                        SchematicItem::Digit(c.to_digit(10).unwrap()),
                    );
                }
                symbol => {
                    engine_schematic_items.insert(
                        Coordinate(x as u32, y as u32),
                        SchematicItem::Symbol(symbol),
                    );
                }
            });
        x_max = schematic_line.len() - 1;
        y_max = y;
    });
    EngineSchematic {
        max_coords: Coordinate(x_max as u32, y_max as u32),
        items: engine_schematic_items,
    }
}

pub fn part1(engine_schematic: &EngineSchematic) -> u32 {
    engine_schematic
        .items
        .iter()
        .filter(|(_, item)| matches!(item, SchematicItem::Symbol(_)))
        .flat_map(|(coordinate, _)| get_adjacent_numbers(coordinate, engine_schematic))
        .sum()
}

pub fn part2(engine_schematic: &EngineSchematic) -> u32 {
    engine_schematic
        .items
        .iter()
        .filter(|(_, item)| matches!(item, SchematicItem::Symbol('*')))
        .map(|(coordinate, _)| get_adjacent_numbers(coordinate, engine_schematic))
        .filter_map(|nums| {
            if nums.len() == 2 {
                Some(nums[0] * nums[1])
            } else {
                None
            }
        })
        .sum()
}

fn get_adjacent_numbers(coordinate: &Coordinate, engine_schematic: &EngineSchematic) -> Vec<u32> {
    let adjacent_coordinates = get_adjacent_coordinates(coordinate, &engine_schematic.max_coords);
    let digit_coordinates = adjacent_coordinates
        .iter()
        .filter(|coord| engine_schematic.is_digit(coord))
        .collect();
    extract_numbers(digit_coordinates, engine_schematic)
}

fn extract_numbers(
    digit_coordinates: Vec<&Coordinate>,
    engine_schematic: &EngineSchematic,
) -> Vec<u32> {
    let mut visited = HashSet::new();
    let mut nums = Vec::new();
    for Coordinate(cx, cy) in digit_coordinates {
        if visited.contains(&(*cx, *cy)) {
            continue;
        }
        visited.insert((*cx, *cy));
        let mut num_str = String::new();
        // Collect digits on left
        let mut x = *cx;
        while let Some(SchematicItem::Digit(d)) = engine_schematic.get_item(x, *cy) {
            visited.insert((x.to_owned(), *cy));
            num_str = format!("{}{}", d, num_str);
            if x > 0 {
                x -= 1;
            } else {
                break;
            }
        }
        // Collect digits on the right
        let max_x = engine_schematic.max_coords.0;
        let mut x = *cx + 1; // could overflow but won't matter, at least yet, so keeping symmetry
        while let Some(SchematicItem::Digit(d)) = engine_schematic.get_item(x, *cy) {
            visited.insert((x.to_owned(), *cy));
            num_str = format!("{}{}", num_str, d);
            if x < max_x {
                x += 1;
            } else {
                break;
            }
        }
        nums.push(
            num_str
                .parse()
                .expect("Digits appended should form a number"),
        );
    }
    nums
}

fn get_adjacent_coordinates(coordinate: &Coordinate, max_coords: &Coordinate) -> Vec<Coordinate> {
    let Coordinate(x, y) = coordinate;
    let Coordinate(max_x, max_y) = max_coords;
    let x_possibilities = match *x {
        0 => vec![0, 1],
        x if x == *max_x => vec![max_x - 1, *max_x],
        other => vec![other - 1, other, other + 1],
    };
    let y_possibilities = match *y {
        0 => vec![0, 1],
        y if y == *max_y => vec![max_y - 1, *max_y],
        other => vec![other - 1, other, other + 1],
    };
    x_possibilities
        .into_iter()
        .flat_map(|x| y_possibilities.iter().map(move |y| Coordinate(x, *y)))
        // filter out num's own overlapping coordinates
        .filter(|Coordinate(px, py)| !(py == y && px == x))
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;
    use indoc::indoc;

    #[test]
    fn generator_example() {
        let input = indoc! {"
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        "};
        let mut expected_items = HashMap::new();
        expected_items.insert(Coordinate(0, 0), SchematicItem::Digit(4));
        expected_items.insert(Coordinate(1, 0), SchematicItem::Digit(6));
        expected_items.insert(Coordinate(2, 0), SchematicItem::Digit(7));
        expected_items.insert(Coordinate(5, 0), SchematicItem::Digit(1));
        expected_items.insert(Coordinate(6, 0), SchematicItem::Digit(1));
        expected_items.insert(Coordinate(7, 0), SchematicItem::Digit(4));
        expected_items.insert(Coordinate(3, 1), SchematicItem::Symbol('*'));
        expected_items.insert(Coordinate(2, 2), SchematicItem::Digit(3));
        expected_items.insert(Coordinate(3, 2), SchematicItem::Digit(5));
        expected_items.insert(Coordinate(6, 2), SchematicItem::Digit(6));
        expected_items.insert(Coordinate(7, 2), SchematicItem::Digit(3));
        expected_items.insert(Coordinate(8, 2), SchematicItem::Digit(3));
        expected_items.insert(Coordinate(6, 3), SchematicItem::Symbol('#'));
        expected_items.insert(Coordinate(0, 4), SchematicItem::Digit(6));
        expected_items.insert(Coordinate(1, 4), SchematicItem::Digit(1));
        expected_items.insert(Coordinate(2, 4), SchematicItem::Digit(7));
        expected_items.insert(Coordinate(3, 4), SchematicItem::Symbol('*'));
        expected_items.insert(Coordinate(5, 5), SchematicItem::Symbol('+'));
        expected_items.insert(Coordinate(7, 5), SchematicItem::Digit(5));
        expected_items.insert(Coordinate(8, 5), SchematicItem::Digit(8));
        expected_items.insert(Coordinate(2, 6), SchematicItem::Digit(5));
        expected_items.insert(Coordinate(3, 6), SchematicItem::Digit(9));
        expected_items.insert(Coordinate(4, 6), SchematicItem::Digit(2));
        expected_items.insert(Coordinate(6, 7), SchematicItem::Digit(7));
        expected_items.insert(Coordinate(7, 7), SchematicItem::Digit(5));
        expected_items.insert(Coordinate(8, 7), SchematicItem::Digit(5));
        expected_items.insert(Coordinate(3, 8), SchematicItem::Symbol('$'));
        expected_items.insert(Coordinate(5, 8), SchematicItem::Symbol('*'));
        expected_items.insert(Coordinate(1, 9), SchematicItem::Digit(6));
        expected_items.insert(Coordinate(2, 9), SchematicItem::Digit(6));
        expected_items.insert(Coordinate(3, 9), SchematicItem::Digit(4));
        expected_items.insert(Coordinate(5, 9), SchematicItem::Digit(5));
        expected_items.insert(Coordinate(6, 9), SchematicItem::Digit(9));
        expected_items.insert(Coordinate(7, 9), SchematicItem::Digit(8));
        let schematic = generator(input);
        println!("{}", schematic);
        assert_eq!(schematic.items, expected_items);
        assert_eq!(schematic.max_coords, Coordinate(9, 9));
    }

    #[test]
    fn part1_example() {
        let input = indoc! {"
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        "};
        let schematic = generator(input);
        println!("{}", schematic);
        assert_eq!(part1(&schematic), 4361);
    }

    #[test]
    fn generator_corner_cases() {
        let input = indoc! {"
            4..134
            ....#.
            23$..1
        "};
        let mut expected_items = HashMap::new();
        expected_items.insert(Coordinate(0, 0), SchematicItem::Digit(4));
        expected_items.insert(Coordinate(3, 0), SchematicItem::Digit(1));
        expected_items.insert(Coordinate(4, 0), SchematicItem::Digit(3));
        expected_items.insert(Coordinate(5, 0), SchematicItem::Digit(4));
        expected_items.insert(Coordinate(4, 1), SchematicItem::Symbol('#'));
        expected_items.insert(Coordinate(0, 2), SchematicItem::Digit(2));
        expected_items.insert(Coordinate(1, 2), SchematicItem::Digit(3));
        expected_items.insert(Coordinate(2, 2), SchematicItem::Symbol('$'));
        expected_items.insert(Coordinate(5, 2), SchematicItem::Digit(1));
        let schematic = generator(input);
        println!("{}", schematic);
        assert_eq!(schematic.items, expected_items);
        assert_eq!(schematic.max_coords, Coordinate(5, 2));
    }

    #[test]
    fn part1_corner_cases() {
        let input = indoc! {"
            3..21
            ...*1
        "};
        assert_eq!(part1(&generator(input)), 21 + 1);
    }

    #[test]
    fn part1_zero_case() {
        let input = indoc! {"
            3..20
            ...*1
        "};
        assert_eq!(part1(&generator(input)), 20 + 1);
    }

    #[test]
    fn generator_trailing_newline_case() {
        let input = indoc! {"
            4..134
            ....#."};
        let mut expected_items = HashMap::new();
        expected_items.insert(Coordinate(0, 0), SchematicItem::Digit(4));
        expected_items.insert(Coordinate(3, 0), SchematicItem::Digit(1));
        expected_items.insert(Coordinate(4, 0), SchematicItem::Digit(3));
        expected_items.insert(Coordinate(5, 0), SchematicItem::Digit(4));
        expected_items.insert(Coordinate(4, 1), SchematicItem::Symbol('#'));
        let schematic = generator(input);
        println!("{}", schematic);
        assert_eq!(schematic.items, expected_items);
        assert_eq!(schematic.max_coords, Coordinate(5, 1));
    }

    #[test]
    fn part2_example() {
        let input = indoc! {"
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        "};
        let schematic = generator(input);
        println!("{}", schematic);
        assert_eq!(part2(&schematic), 467835);
    }
}
