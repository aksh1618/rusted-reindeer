use std::{
    collections::{HashMap, HashSet},
    iter::successors,
};

use itertools::Itertools;

type Coordinate = (i32, i32);
type Map = HashMap<Coordinate, Tile>;

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

#[derive(Debug)]
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

#[derive(PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn next_coord(&self, (x, y): Coordinate) -> Coordinate {
        match self {
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
        }
    }
}

#[allow(clippy::cast_possible_wrap)]
pub fn generator(input: &str) -> (Coordinate, Map) {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
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
            ((0, 0), Map::new()),
            |(animal_coord, mut map), (coord, tile)| {
                let animal_coord = if matches!(tile.tile_type, TileType::Animal) {
                    coord
                } else {
                    animal_coord
                };
                map.insert(coord, tile);
                (animal_coord, map)
            },
        )
}

fn traverse_loop_in_map(loop_coord: Coordinate, map: &Map) -> u32 {
    let neighbouring_loop_pipe_coordinates = [
        Direction::Left,
        Direction::Right,
        Direction::Up,
        Direction::Down,
    ]
    .iter()
    .map(|direction| direction.next_coord(loop_coord))
    .filter(|next_coord| {
        map.get(next_coord)
            .and_then(Tile::connecting_coords_if_pipe)
            .filter(|coords| coords.contains(&loop_coord))
            .is_some()
    })
    .collect_vec();

    let mut visited: HashSet<Coordinate> = HashSet::new();
    visited.extend(neighbouring_loop_pipe_coordinates.iter());

    successors(Some(neighbouring_loop_pipe_coordinates), |coords| {
        if coords.len() == 2 && coords[0] == coords[1] {
            None
        } else {
            let next_pipes = coords
                .iter()
                .filter_map(|coord| map.get(coord).and_then(Tile::connecting_coords_if_pipe))
                .flatten()
                .filter(|coord| {
                    matches!(
                        map.get(coord).map(|tile| &tile.tile_type),
                        Some(TileType::Pipe(_))
                    )
                })
                .filter(|coord| !visited.contains(coord))
                .collect_vec();
            visited.extend(next_pipes.iter());
            // print!("{next_pipes:?}\n\n");
            Some(next_pipes)
        }
    })
    .count() as u32
}

pub fn part1((animal_coord, map): &(Coordinate, Map)) -> u32 {
    traverse_loop_in_map(*animal_coord, map)
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
}
