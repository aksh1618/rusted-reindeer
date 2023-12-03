use getset::Getters;
use std::collections::HashMap;

#[derive(Getters)]
pub struct Game {
    #[getset(get)]
    id: u32,
    reveals: Vec<CubesReveal>,
}

pub struct CubesReveal {
    red_cubes: u32,
    green_cubes: u32,
    blue_cubes: u32,
}

impl CubesReveal {
    fn from_reveal_string(reveal_string: &str) -> Self {
        let cube_counts = reveal_string
            .trim()
            .split(',')
            .map(|cube_reveal| {
                let (count, color) = cube_reveal
                    .trim()
                    .split_once(' ')
                    .expect("Cube reveal should be space separated");
                let count = count
                    .parse()
                    .expect("Count in cube reveal should be a number");
                (color, count)
            })
            .collect::<HashMap<&str, u32>>();
        Self {
            red_cubes: *cube_counts.get("red").unwrap_or(&0),
            green_cubes: *cube_counts.get("green").unwrap_or(&0),
            blue_cubes: *cube_counts.get("blue").unwrap_or(&0),
        }
    }
}

pub fn generator(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|game_record| {
            let (game_id, game_reveals) = game_record
                .split_once(':')
                .expect("Record of game should be in the expected format");
            let id = game_id
                .strip_prefix("Game ")
                .expect("Record of game should start with `Game `")
                .trim()
                .parse::<u32>()
                .expect("Record of game should have an ID number");
            let reveals: Vec<CubesReveal> = game_reveals
                .trim()
                .split(';')
                .map(|cubes_reveal| CubesReveal::from_reveal_string(cubes_reveal.trim()))
                .collect();
            Game { id, reveals }
        })
        .collect()
}

pub fn part1(games: &[Game]) -> u32 {
    games.iter().filter(is_game_possible).map(Game::id).sum()
}

fn is_game_possible(game: &&Game) -> bool {
    game.reveals
        .iter()
        .all(|reveal| reveal.red_cubes <= 12 && reveal.green_cubes <= 13 && reveal.blue_cubes <= 14)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn part1_example() {
        let input = indoc! {"
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "};
        assert_eq!(part1(&generator(input)), 8);
    }
}
