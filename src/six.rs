use array2d::Array2D;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn rotate(&self) -> Direction {
        match &self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

pub fn run() {
    let path = Path::new("data/six");
    // Still contains the guard
    let starting_map = parse_to_arr(path);

    let simulated = simulate_guard(&starting_map);
    let visited = count_visited(&simulated);
    println!("Visited {} squares", visited);

    println!("{} possible blockages", part_two(&starting_map));
}

fn parse_to_arr(path: &Path) -> Array2D<char> {
    let file = File::open(&path).expect("Need input file");
    let lines = io::BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    return Array2D::from_rows(&lines).expect("Can't do much without a map");
}

fn simulate_guard(starting_map: &Array2D<char>) -> Array2D<char> {
    let mut working_map = starting_map.clone();
    let mut guard_direction = Direction::Up;
    // row, col
    let mut guard_pos = get_guard_pos(&working_map);

    loop {
        working_map.set(guard_pos.0, guard_pos.1, 'X').unwrap();
        if let Some((next_direction, next_pos)) = get_next(&working_map, guard_direction, guard_pos) {
            guard_pos = next_pos;
            guard_direction = next_direction;
        } else {
            return working_map;
        }
    }
}

fn part_two(starting_map: &Array2D<char>) -> usize {
    let mut working_map = starting_map.clone();
    let mut guard_direction = Direction::Up;
    // row, col
    let initial_guard_pos = get_guard_pos(&working_map);
    let mut guard_pos = initial_guard_pos.clone();
    let mut positions = 0;

    loop {
        working_map.set(guard_pos.0, guard_pos.1, 'X').unwrap();
        if let Some((next_direction, next_pos)) = get_next(&working_map, guard_direction, guard_pos) {
            guard_pos = next_pos;
            guard_direction = next_direction;
            if *working_map.get(next_pos.0, next_pos.1).expect("Impossible") == '.' {
                working_map.set(next_pos.0, next_pos.1, '#').unwrap();
                if does_loop(&working_map, initial_guard_pos) {
                    positions += 1;
                }
                working_map.set(next_pos.0, next_pos.1, '.').unwrap();
            }
        } else {
            return positions;
        }
    }
}

fn does_loop(map: &Array2D<char>, starting_pos: (usize, usize)) -> bool {
    let mut working_map = map.clone();
    let mut guard_direction = Direction::Up;
    // row, col
    let mut guard_pos = starting_pos;
    let mut obstacles: Vec<((usize, usize), Direction)> = Vec::new();

    loop {
        working_map.set(guard_pos.0, guard_pos.1, 'X').unwrap();
        if let Some((next_direction, next_pos)) = get_next(&working_map, guard_direction, guard_pos) {
            if next_pos == guard_pos {
                if obstacles.contains(&(next_pos, next_direction)) {
                    // We are in a loop
                    return true;
                } else {
                    obstacles.push((next_pos, next_direction));
                }
            }
            guard_pos = next_pos;
            guard_direction = next_direction;
        } else {
            return false;
        }
    }
}

fn get_next(map: &Array2D<char>, direction: Direction, pos: (usize, usize)) -> Option<(Direction, (usize, usize))> {
    if facing(map, pos, direction)? != '#' {
        Some((direction, proceed(direction, pos)))
    } else {
        Some((direction.rotate(), pos))
    }
}

fn proceed(direction: Direction, pos: (usize, usize)) -> (usize, usize) {
    match direction {
        Direction::Up => (pos.0 - 1, pos.1),
        Direction::Down => (pos.0 + 1, pos.1),
        Direction::Left => (pos.0, pos.1 - 1),
        Direction::Right => (pos.0, pos.1 + 1),
    }
}

fn facing(map: &Array2D<char>, pos: (usize, usize), direction: Direction) -> Option<char> {
    match direction {
        Direction::Up => map.get(pos.0.checked_sub(1)?, pos.1).copied(),
        Direction::Down => map.get(pos.0 + 1, pos.1).copied(),
        Direction::Left => map.get(pos.0, pos.1.checked_sub(1)?).copied(),
        Direction::Right => map.get(pos.0, pos.1 + 1).copied(),
    }
}

fn get_guard_pos(map: &Array2D<char>) -> (usize, usize) {
    for (row_id, row) in map.rows_iter().enumerate() {
        for (col_id, el) in row.enumerate() {
            if *el == '^' {
                return (row_id, col_id);
            }
        }
    }
    panic!()
}

fn count_visited(map: &Array2D<char>) -> usize {
    map.elements_column_major_iter()
        .filter(|c| **c == 'X')
        .count()
}

#[allow(dead_code)]
fn print_map(map: &Array2D<char>) {
    let folded = map
        .rows_iter()
        .map(|row| row.collect::<String>())
        .fold(String::new(), |acc: String, x| format!("{}{}\n", acc, x));
    println!("{}", folded)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part_one() {
        let lines = INPUT
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let starting_map: Array2D<char> = Array2D::from_rows(&lines).unwrap();

        let simulated = simulate_guard(&starting_map);
        print_map(&simulated);
        let visited = count_visited(&simulated);

        assert_eq!(visited, 41);
    }

    #[test]
    fn test_part_two() {
        let lines = INPUT
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let starting_map: Array2D<char> = Array2D::from_rows(&lines).unwrap();


        assert_eq!(part_two(&starting_map), 6);
    }
}
