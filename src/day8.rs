use std::collections::HashMap;

#[derive(Debug)]
pub struct Node {
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
                left_element: left_element.to_string(),
                right_element: right_element.to_string(),
            },
        );
    }
    (instructions, nodes)
}

pub fn part1((instructions, network): &(Instructions, Network)) -> u32 {
    let mut cur_node = &network["AAA"];
    let mut instructions_followed = 0;
    for instruction in instructions.iter().cycle() {
        instructions_followed += 1;
        let next_element = match instruction {
            Instruction::L => &cur_node.left_element,
            Instruction::R => &cur_node.right_element,
        };
        if next_element == "ZZZ" {
            break;
        }
        cur_node = &network[next_element];
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

    #[test]
    fn part1_example() {
        println!("{:?}", generator(EXAMPLE_INPUT_1));
        assert_eq!(part1(&generator(EXAMPLE_INPUT_1)), 2);
        assert_eq!(part1(&generator(EXAMPLE_INPUT_2)), 6);
    }
}
