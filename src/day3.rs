use std::{collections::HashMap, fmt};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Coordinate(u32, u32);
pub struct EngineSchematic {
    max_coords: Coordinate,
    items: HashMap<Coordinate, SchematicItem>,
}

impl EngineSchematic {
    fn is_symbol(&self, coordinate: &Coordinate) -> bool {
        self.items
            .get(coordinate)
            .filter(|item| matches!(item, SchematicItem::Symbol(_)))
            .is_some()
    }
}

impl fmt::Display for EngineSchematic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Coordinate(max_x, max_y) = self.max_coords;
        let mut output = String::new();
        for y in 0..=max_y {
            let mut dots_to_skip = 0;
            for x in 0..=max_x {
                if let Some(schematic_item) = self.items.get(&Coordinate(x, y)) {
                    match schematic_item {
                        SchematicItem::Symbol(sym) => output.push(*sym),
                        SchematicItem::Number(num) => {
                            let num_str = num.to_string();
                            output.push_str(num_str.as_str());
                            dots_to_skip = num_str.len() - 1;
                        }
                    }
                } else if dots_to_skip > 0 {
                    dots_to_skip -= 1;
                } else {
                    output.push('.');
                }
            }
            output.push('\n');
        }
        write!(f, "{}", output)
    }
}

#[derive(Debug, PartialEq)]
pub enum SchematicItem {
    Symbol(char),
    Number(u32),
}

pub fn generator(input: &str) -> EngineSchematic {
    let mut engine_schematic_items = HashMap::new();
    let lines = input.lines();
    lines.enumerate().for_each(|(y, schematic_line)| {
        let mut chars = schematic_line.chars().enumerate();
        let (x, c) = chars.next().expect("Input line should not be empty");
        parse_schematic_item(c, &mut chars, &mut engine_schematic_items, x, y, 0);
    });
    let input_len_with_trailing_newline =
        input.strip_suffix('\n').unwrap_or(input).len() as u32 + 1;
    let line_len = input.lines().next().unwrap().len() as u32;
    let x_max = line_len - 1;
    // let y_max = (len as f32 / (x_max + 2) as f32).ceil() as u32 - 1;
    let y_max = ((input_len_with_trailing_newline) / (line_len + 1)) - 1;
    EngineSchematic {
        max_coords: Coordinate(x_max, y_max),
        items: engine_schematic_items,
    }
}

fn parse_schematic_item(
    c: char,
    chars: &mut std::iter::Enumerate<std::str::Chars<'_>>,
    engine_schematic_items: &mut HashMap<Coordinate, SchematicItem>,
    x: usize,
    y: usize,
    num_so_far: u32,
) {
    match c {
        '.' => {
            if num_so_far > 0 {
                engine_schematic_items.insert(
                    Coordinate((x - num_so_far.to_string().len()) as u32, y as u32),
                    SchematicItem::Number(num_so_far),
                );
            }
            while let Some((x, c)) = chars.next() {
                parse_schematic_item(c, chars, engine_schematic_items, x, y, 0);
            }
        }
        '0'..='9' => {
            let digit = c.to_digit(10).unwrap();
            if let Some((x, c)) = chars.next() {
                parse_schematic_item(
                    c,
                    chars,
                    engine_schematic_items,
                    x,
                    y,
                    num_so_far * 10 + digit,
                );
            } else if num_so_far > 0 {
                engine_schematic_items.insert(
                    Coordinate((x - num_so_far.to_string().len()) as u32, y as u32),
                    SchematicItem::Number(num_so_far * 10 + digit),
                );
            } else {
                engine_schematic_items
                    .insert(Coordinate(x as u32, y as u32), SchematicItem::Number(digit));
            }
        }
        symbol => {
            engine_schematic_items.insert(
                Coordinate(x as u32, y as u32),
                SchematicItem::Symbol(symbol),
            );
            if num_so_far > 0 {
                engine_schematic_items.insert(
                    Coordinate((x - num_so_far.to_string().len()) as u32, y as u32),
                    SchematicItem::Number(num_so_far),
                );
            }
            while let Some((x, c)) = chars.next() {
                parse_schematic_item(c, chars, engine_schematic_items, x, y, 0);
            }
        }
    };
}

pub fn part1(engine_schematic: &EngineSchematic) -> u32 {
    // print!("{}", engine_schematic);
    engine_schematic
        .items
        .iter()
        .filter(|(coordinate, schematic_item)| {
            is_part_number(schematic_item, coordinate, engine_schematic)
        })
        .map(|(_, schematic_item)| {
            if let SchematicItem::Number(num) = schematic_item {
                num
            } else {
                &0
            }
        })
        .sum()
}

fn is_part_number(
    schematic_item: &SchematicItem,
    coordinate: &Coordinate,
    engine_schematic: &EngineSchematic,
) -> bool {
    if let SchematicItem::Number(num) = schematic_item {
        get_adjacent_coordinates(num, coordinate, &engine_schematic.max_coords)
            // get_adjacent_coordinates_take2(num, coordinate)
            .iter()
            .any(|adjacent_coordinate| engine_schematic.is_symbol(adjacent_coordinate))
    } else {
        false
    }
}

fn get_adjacent_coordinates(
    num: &u32,
    coordinate: &Coordinate,
    max_coords: &Coordinate,
) -> Vec<Coordinate> {
    let num_digits = num.to_string().len() as u32;
    let Coordinate(x, y) = coordinate;
    let Coordinate(max_x, max_y) = max_coords;
    let effective_max_x = max_x - num_digits;
    let x_possibilities = match *x {
        0 => 0..=num_digits,
        x if x == effective_max_x => effective_max_x - 1..=*max_x,
        other => other - 1..=other + num_digits,
    };
    let y_possibilities = match *y {
        0 => vec![0, 1],
        y if y == *max_y => vec![*max_y, max_y - 1],
        other => vec![other - 1, other, other + 1],
    };
    x_possibilities
        .into_iter()
        .flat_map(|x| y_possibilities.iter().map(move |y| Coordinate(x, *y)))
        // filter out num's own overlapping coordinates
        .filter(|Coordinate(px, py)| !(py == y && (x..&(x + num_digits - 1)).contains(&px)))
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
        expected_items.insert(Coordinate(0, 0), SchematicItem::Number(467));
        expected_items.insert(Coordinate(5, 0), SchematicItem::Number(114));
        expected_items.insert(Coordinate(3, 1), SchematicItem::Symbol('*'));
        expected_items.insert(Coordinate(2, 2), SchematicItem::Number(35));
        expected_items.insert(Coordinate(6, 2), SchematicItem::Number(633));
        expected_items.insert(Coordinate(6, 3), SchematicItem::Symbol('#'));
        expected_items.insert(Coordinate(0, 4), SchematicItem::Number(617));
        expected_items.insert(Coordinate(3, 4), SchematicItem::Symbol('*'));
        expected_items.insert(Coordinate(5, 5), SchematicItem::Symbol('+'));
        expected_items.insert(Coordinate(7, 5), SchematicItem::Number(58));
        expected_items.insert(Coordinate(2, 6), SchematicItem::Number(592));
        expected_items.insert(Coordinate(6, 7), SchematicItem::Number(755));
        expected_items.insert(Coordinate(3, 8), SchematicItem::Symbol('$'));
        expected_items.insert(Coordinate(5, 8), SchematicItem::Symbol('*'));
        expected_items.insert(Coordinate(1, 9), SchematicItem::Number(664));
        expected_items.insert(Coordinate(5, 9), SchematicItem::Number(598));
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
        assert_eq!(part1(&generator(input)), 4361);
    }

    #[test]
    fn generator_corner_cases() {
        let input = indoc! {"
            4..134
            ....#.
            23$..1
        "};
        let mut expected_items = HashMap::new();
        expected_items.insert(Coordinate(0, 0), SchematicItem::Number(4));
        expected_items.insert(Coordinate(3, 0), SchematicItem::Number(134));
        expected_items.insert(Coordinate(4, 1), SchematicItem::Symbol('#'));
        expected_items.insert(Coordinate(0, 2), SchematicItem::Number(23));
        expected_items.insert(Coordinate(2, 2), SchematicItem::Symbol('$'));
        expected_items.insert(Coordinate(5, 2), SchematicItem::Number(1));
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
        expected_items.insert(Coordinate(0, 0), SchematicItem::Number(4));
        expected_items.insert(Coordinate(3, 0), SchematicItem::Number(134));
        expected_items.insert(Coordinate(4, 1), SchematicItem::Symbol('#'));
        let schematic = generator(input);
        println!("{}", schematic);
        assert_eq!(schematic.items, expected_items);
        assert_eq!(schematic.max_coords, Coordinate(5, 1));
    }
}
