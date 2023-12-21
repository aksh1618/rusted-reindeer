use std::{
    collections::{HashMap, HashSet},
    iter::successors,
};

use itertools::Itertools;
use strum::{EnumIter, IntoEnumIterator};

type Coordinate = (i32, i32);

#[derive(Debug)]
pub struct Map {
    tiles: HashMap<Coordinate, Tile>,
    animal_coords: Coordinate,
    max_coords: Coordinate,
}

#[derive(Debug)]
pub struct Tile {
    coordinate: Coordinate,
    tile_type: TileType,
}

impl Tile {
    fn connecting_coords_if_pipe(&self) -> Option<[Coordinate; 2]> {
        match &self.tile_type {
            TileType::Pipe(pipe) => Some(pipe.connecting_coords(self.coordinate)),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum TileType {
    Pipe(Pipe),
    Ground,
    Animal,
}

impl TileType {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Ground,
            'S' => Self::Animal,
            other => Self::Pipe(Pipe::from_char(other)),
        }
    }
}

#[derive(Debug, EnumIter)]
pub enum Pipe {
    Horizontal,
    Vertical,
    BottomLeft,
    TopLeft,
    BottomRight,
    TopRight,
}

impl Pipe {
    fn from_char(c: char) -> Self {
        match c {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::BottomLeft,
            'J' => Self::BottomRight,
            '7' => Self::TopRight,
            'F' => Self::TopLeft,
            _ => unreachable!(),
        }
    }

    fn from_connecting_directions(directions: [Direction; 2]) -> Self {
        Pipe::iter()
            .find(|pipe| {
                let connecting_directions = &pipe.connecting_directions();
                connecting_directions.contains(&directions[0])
                    && connecting_directions.contains(&directions[1])
            })
            .expect("there must exist a pipe connecting two pipes in a loop")
    }

    fn connecting_directions(&self) -> [Direction; 2] {
        match self {
            Pipe::Horizontal => [Direction::Left, Direction::Right],
            Pipe::Vertical => [Direction::Up, Direction::Down],
            Pipe::BottomLeft => [Direction::Up, Direction::Right],
            Pipe::TopLeft => [Direction::Down, Direction::Right],
            Pipe::BottomRight => [Direction::Left, Direction::Up],
            Pipe::TopRight => [Direction::Left, Direction::Down],
        }
    }

    fn connecting_coords(&self, coordinate: Coordinate) -> [Coordinate; 2] {
        let [d1, d2] = self.connecting_directions();
        [d1.next_coord(coordinate), d2.next_coord(coordinate)]
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn next_coord(self, (x, y): Coordinate) -> Coordinate {
        match self {
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
        }
    }
}

#[allow(clippy::cast_possible_wrap)]
pub fn generator(input: &str) -> Map {
    let (mut x_max, mut y_max) = (0, 0);
    let (animal_coords, tiles) = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            x_max = line.len() - 1;
            y_max = y;
            line.chars().enumerate().map(move |(x, char)| {
                let coord = (x as i32, y as i32);
                (
                    coord,
                    Tile {
                        coordinate: coord,
                        tile_type: TileType::from_char(char),
                    },
                )
            })
        })
        .fold(
            ((0, 0), HashMap::new()),
            |(animal_coord, mut map), (coord, tile)| {
                let animal_coord = if matches!(tile.tile_type, TileType::Animal) {
                    coord
                } else {
                    animal_coord
                };
                map.insert(coord, tile);
                (animal_coord, map)
            },
        );
    let max_coords = (x_max as i32, y_max as i32);
    Map {
        tiles,
        animal_coords,
        max_coords,
    }
}

fn trace_main_loop(map: &Map) -> HashSet<Coordinate> {
    let (_, animal_connecting_pipes_coords): (Vec<_>, Vec<_>) =
        get_connecting_pipes(map.animal_coords, map)
            .iter()
            .copied()
            .unzip();

    let mut visited: HashSet<Coordinate> = HashSet::new();
    visited.insert(map.animal_coords);
    visited.extend(animal_connecting_pipes_coords.iter());

    successors(Some(animal_connecting_pipes_coords), |coords| {
        if coords.len() == 2 && coords[0] == coords[1] {
            None
        } else {
            let next_connecting_pipes = coords
                .iter()
                .filter_map(|coord| {
                    map.tiles
                        .get(coord)
                        .and_then(Tile::connecting_coords_if_pipe)
                })
                .flatten()
                .filter(|coord| {
                    matches!(
                        map.tiles.get(coord).map(|tile| &tile.tile_type),
                        Some(TileType::Pipe(_))
                    )
                })
                .filter(|coord| !visited.contains(coord))
                .collect_vec();
            visited.extend(next_connecting_pipes.iter());
            // print!("{next_pipes:?}\n\n");
            Some(next_connecting_pipes)
        }
    })
    .last();
    visited
}

fn get_connecting_pipes(loop_coords: Coordinate, map: &Map) -> [(Direction, Coordinate); 2] {
    let neighbours = [
        Direction::Left,
        Direction::Right,
        Direction::Up,
        Direction::Down,
    ]
    .iter()
    .map(|direction| (*direction, direction.next_coord(loop_coords)))
    .filter(|(_, next_coord)| {
        map.tiles
            .get(next_coord)
            .and_then(Tile::connecting_coords_if_pipe)
            .filter(|coords| coords.contains(&loop_coords))
            .is_some()
    })
    .collect_vec();
    assert_eq!(neighbours.len(), 2);
    [neighbours[0], neighbours[1]]
}

pub fn part1(map: &Map) -> u32 {
    (trace_main_loop(map).len() / 2) as u32
}

fn infer_animal_pipe(map: &Map) -> Pipe {
    let (animal_connecting_pipe_directions, _): (Vec<_>, Vec<_>) =
        get_connecting_pipes(map.animal_coords, map)
            .iter()
            .copied()
            .unzip();
    Pipe::from_connecting_directions([
        animal_connecting_pipe_directions[0],
        animal_connecting_pipe_directions[1],
    ])
}

/// Uses ray casting to count the points inside the main loop.
/// We need to choose either up or down as the direction which
/// we will consider as intersection to account for different
/// combinations of bend pipes:
///
/// ```text
///  outside F---7 outside
///  outside F---J inside
///  outside L---J outside
///  outside L---7 inside
/// ```
fn count_points_inside_main_loop(map: &Map) -> u32 {
    let main_loop_coords = trace_main_loop(map);
    let (x_max, y_max) = map.max_coords;
    let mut num_inside_points = 0;
    // row to cast ray
    for y in 0..=y_max {
        let mut inside = false;
        // ray casting
        for x in 0..=x_max {
            let coords = (x, y);
            let is_loop_tile = main_loop_coords.contains(&coords);
            let tile_type = &map.tiles[&coords].tile_type;
            match tile_type {
                TileType::Animal => {
                    let animal_pipe = infer_animal_pipe(map);
                    if animal_pipe.connecting_directions().contains(&Direction::Up) {
                        // intersection
                        inside = !inside;
                    }
                }
                TileType::Pipe(pipe) if is_loop_tile => {
                    if pipe.connecting_directions().contains(&Direction::Up) {
                        // intersection
                        inside = !inside;
                    }
                }
                _ => {
                    if inside {
                        num_inside_points += 1;
                    }
                }
            }
        }
    }
    num_inside_points
}

pub fn part2(map: &Map) -> u32 {
    count_points_inside_main_loop(map)
}

#[cfg(test)]
mod tests {

    use super::*;
    use indoc::indoc;

    const EXAMPLE_INPUT_1: &str = indoc! {"
        -L|F7
        7S-7|
        L|7||
        -L-J|
        L|-JF
    "};

    const EXAMPLE_INPUT_2: &str = indoc! {"
        7-F7-
        .FJ|7
        SJLL7
        |F--J
        LJ.LJ
    "};

    #[test]
    fn part1_example() {
        // println!("{:?}", generator(EXAMPLE_INPUT_1));
        // println!("{:?}", generator(EXAMPLE_INPUT_2));
        assert_eq!(part1(&generator(EXAMPLE_INPUT_1)), 4);
        assert_eq!(part1(&generator(EXAMPLE_INPUT_2)), 8);
    }

    #[test]
    fn part2_example() {
        let map = indoc! {"
            .....
            .S-7.
            .|.|.
            .L-J.
            .....
        "};
        assert_eq!(part2(&generator(map)), 1);

        let map = indoc! {"
            ...........
            .S-------7.
            .|F-----7|.
            .||.....||.
            .||.....||.
            .|L-7.F-J|.
            .|..|.|..|.
            .L--J.L--J.
            ...........
        "};
        assert_eq!(part2(&generator(map)), 4);

        let map = indoc! {"
            ...........
            .F--7F---7.
            .|..|LS7.|.
            .|..|..|.|.
            .|..|..|.|.
            .|..L--J.|.
            .|.......|.
            .L-------J.
            ...........
        "};
        assert_eq!(part2(&generator(map)), 19);

        let map = indoc! {"
            .F----7F7F7F7F-7....
            .|F--7||||||||FJ....
            .||.FJ||||||||L7....
            FJL7L7LJLJ||LJ.L-7..
            L--J.L7...LJS7F-7L7.
            ....F-J..F7FJ|L7L7L7
            ....L7.F7||L7|.L7L7|
            .....|FJLJ|FJ|F7|.LJ
            ....FJL-7.||.||||...
            ....L---J.LJ.LJLJ...
        "};
        assert_eq!(part2(&generator(map)), 8);

        let map = indoc! {"
            FF7FSF7F7F7F7F7F---7
            L|LJ||||||||||||F--J
            FL-7LJLJ||||||LJL-77
            F--JF--7||LJLJ7F7FJ-
            L---JF-JLJ.||-FJLJJ7
            |F|F-JF---7F7-L7L|7|
            |FFJF7L7F-JF7|JL---7
            7-L-JL7||F7|L7F-7F7|
            L.L7LFJ|||||FJL7||LJ
            L7JLJL-JLJLJL--JLJ.L
        "};
        assert_eq!(part2(&generator(map)), 10);

        let map = indoc! {"
            FF7FSF7F7F7F7F7F---7
            L|LJ||||||||||||F--J
            FL-7LJLJ||||||LJL-77
            F--JF--7||LJLJ.F7FJ-
            L---JF-JLJ....FJLJJ7
            |F|F-JF---7...L7L|7|
            |FFJF7L7F-JF7..L---7
            7-L-JL7||F7|L7F-7F7|
            L.L7LFJ|||||FJL7||LJ
            L7JLJL-JLJLJL--JLJ.L
        "};
        assert_eq!(part2(&generator(map)), 10);
    }
}
