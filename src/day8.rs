use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Node {
    _element: String,
    left_element: String,
    right_element: String,
}

#[derive(Debug)]
pub enum Instruction {
    L,
    R,
}

pub type Network = HashMap<String, Node>;
pub type Instructions = Vec<Instruction>;

#[allow(clippy::unwrap_used)]
pub fn generator(input: &str) -> (Instructions, Network) {
    let mut lines = input.lines();
    let instructions = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => Instruction::L,
            'R' => Instruction::R,
            _ => unreachable!(),
        })
        .collect();
    lines.next();
    let mut nodes = HashMap::new();
    for line in lines {
        let (element, next_elements) = line.split_once(" = ").unwrap();
        let (left_element, right_element) = next_elements
            .strip_prefix('(')
            .unwrap()
            .strip_suffix(')')
            .unwrap()
            .split_once(", ")
            .unwrap();
        nodes.insert(
            element.to_string(),
            Node {
                _element: element.to_string(),
                left_element: left_element.to_string(),
                right_element: right_element.to_string(),
            },
        );
    }
    (instructions, nodes)
}

pub fn count_steps<P>(
    (instructions, network): &(Instructions, Network),
    start_element: &str,
    predicate: &mut P,
) -> u32
where
    P: FnMut(&String) -> bool,
{
    let mut cur_node = &network[start_element];
    let mut instructions_followed = 0;
    for instruction in instructions.iter().cycle() {
        instructions_followed += 1;
        let next_element = match instruction {
            Instruction::L => &cur_node.left_element,
            Instruction::R => &cur_node.right_element,
        };
        if predicate(next_element) {
            break;
        }
        cur_node = &network[next_element];
    }
    instructions_followed
}

pub fn part1(instructions_and_network: &(Instructions, Network)) -> u32 {
    let mut final_step_predicate = |next_element: &_| next_element == "ZZZ";
    count_steps(instructions_and_network, "AAA", &mut final_step_predicate)
}

pub fn part2(instructions_and_network: &(Instructions, Network)) -> u64 {
    instructions_and_network
        .1
        .keys()
        .filter(|key| key.ends_with('A'))
        .map(|key| {
            let mut final_step_predicate = |next_element: &String| next_element.ends_with('Z');
            let steps = count_steps(instructions_and_network, key, &mut final_step_predicate);
            u64::from(steps)
        })
        .reduce(u64::lcm)
        .expect("element ending with Z should exist for each starting element ending with A")
}

trait Arithmetic<T> {
    fn lcm(num1: T, num2: T) -> T;
}

macro_rules! impl_arithmetic_for_usize {
    ($($T:ty)*) => {
        $(
            impl Arithmetic<$T> for $T {
                fn lcm(num1: $T, num2: $T) -> $T {
                    let greater = <$T>::max(num1, num2);
                    let lesser = <$T>::min(num1, num2);
                    (1..=lesser)
                        .find(|i| (greater * i) % lesser == 0)
                        .map(|i| (greater * i))
                        .expect(&format!("lcm should exist for whole numbers of type {}", stringify!($t))[..])
                }
            }
        )*
    };
}

impl_arithmetic_for_usize!(u64);

#[allow(dead_code)]
pub fn part2_naive((instructions, network): &(Instructions, Network)) -> u32 {
    let mut cur_nodes = network
        .iter()
        .filter(|(key, _)| key.ends_with('A'))
        .map(|(_, val)| val)
        .take(2)
        .collect_vec();
    let mut instructions_followed = 0;
    // let total_instructions = instructions.len() as u32;
    // println!("{total_instructions}");
    for instruction in instructions.iter().cycle() {
        instructions_followed += 1;
        let next_elements = cur_nodes
            .iter()
            .map(|cur_node| match instruction {
                Instruction::L => &cur_node.left_element,
                Instruction::R => &cur_node.right_element,
            })
            .collect_vec();
        if next_elements
            .iter()
            .all(|next_element| next_element.ends_with('Z'))
        {
            break;
            // println!("{}", instructions_followed % total_instructions);
        }
        cur_nodes = next_elements
            .iter()
            .map(|next_element| &network[*next_element])
            .collect();
    }
    instructions_followed
}

#[cfg(test)]
mod tests {

    use super::*;
    use indoc::indoc;

    const EXAMPLE_INPUT_1: &str = indoc! {"
        RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)
    "};

    const EXAMPLE_INPUT_2: &str = indoc! {"
        LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)
    "};

    const EXAMPLE_INPUT_3: &str = indoc! {"
        LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)
    "};

    #[test]
    fn part1_example() {
        println!("{:?}", generator(EXAMPLE_INPUT_1));
        assert_eq!(part1(&generator(EXAMPLE_INPUT_1)), 2);
        assert_eq!(part1(&generator(EXAMPLE_INPUT_2)), 6);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2_naive(&generator(EXAMPLE_INPUT_3)), 6);
        assert_eq!(part2(&generator(EXAMPLE_INPUT_3)), 6);
    }
}
