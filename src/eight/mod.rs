use std::collections::HashSet;

use itertools::Itertools;

pub fn run() {
    let input = include_str!("../../data/eight");

    println!("One: {}", part_one(input));
    println!("One: {}", part_two(input));
}

fn part_one(input: &str) -> usize {
    let points = split_layers(input)
        .iter()
        .map(|(f, i)| compute_antinodes(*f, i))
        .fold(HashSet::new(), |mut acc: HashSet<(usize, usize)>, x| {
            acc.extend(&x);
            acc
        });
    points.len()
}

fn part_two(input: &str) -> usize {
    let points = split_layers(input)
        .iter()
        .map(|(f, i)| compute_harmonic(*f, i))
        .fold(HashSet::new(), |mut acc: HashSet<(usize, usize)>, x| {
            acc.extend(&x);
            acc
        });
    points.len()
}

fn split_layers(input: &str) -> Vec<(char, String)> {
    input
        .chars()
        .unique()
        .filter(|x| *x != '.')
        .map(|c| {
            (
                c,
                input
                    .chars()
                    .map(|x| if x != c && x != '\n' { '.' } else { x })
                    .collect::<String>(),
            )
        })
        .collect()
}

fn compute_antinodes(frequency: char, input: &String) -> HashSet<(usize, usize)> {
    let mut antinodes = HashSet::new();
    let mut nodes: HashSet<(usize, usize)> = HashSet::new();

    let height = input.lines().count();
    let width = input.lines().nth(0).unwrap().chars().count();

    for (row_idx, row) in input.lines().enumerate() {
        for (col_idx, c) in row.chars().enumerate() {
            if c != frequency {
                continue;
            }
            let pos = (row_idx as i64, col_idx as i64);

            // Check against every previous node. Following nodes will check against this.
            for node in &nodes {
                let node = (node.0 as i64, node.1 as i64);
                let from_pos = (node.0 - pos.0, node.1 - pos.1);
                let from_node = (pos.0 - node.0, pos.1 - node.1);

                let candidate_one = (pos.0 + from_node.0, pos.1 + from_node.1);
                let candidate_two = (node.0 + from_pos.0, node.1 + from_pos.1);

                if candidate_one.0 >= 0 && candidate_one.0 < height as i64
                    && candidate_one.1 >= 0 && candidate_one.1 < width as i64 {
                        antinodes.insert((candidate_one.0 as usize, candidate_one.1 as usize));
                    }
                if candidate_two.0 >= 0 && candidate_two.0 < height as i64
                    && candidate_two.1 >= 0 && candidate_two.1 < width as i64 {
                        antinodes.insert((candidate_two.0 as usize, candidate_two.1 as usize));
                    }
            }

            nodes.insert((pos.0 as usize, pos.1 as usize));
        }
    }

    antinodes
}

fn compute_harmonic(frequency: char, input: &String) -> HashSet<(usize, usize)> {
    let mut antinodes = HashSet::new();
    let mut nodes: HashSet<(usize, usize)> = HashSet::new();

    let height = input.lines().count();
    let width = input.lines().nth(0).unwrap().chars().count();

    for (row_idx, row) in input.lines().enumerate() {
        for (col_idx, c) in row.chars().enumerate() {
            if c != frequency {
                continue;
            }
            let pos = (row_idx as i64, col_idx as i64);

            // Check against every previous node. Following nodes will check against this.
            for node in &nodes {
                let node = (node.0 as i64, node.1 as i64);
                let from_pos = (node.0 - pos.0, node.1 - pos.1);
                let from_node = (pos.0 - node.0, pos.1 - node.1);

                // Somewhat arbitrary bound, if I bugger the early abort it won't run forever
                for mult in 0..height as i64 {
                    let mut found = false;
                    let from_pos = (from_pos.0 * mult, from_pos.1 * mult);
                    let from_node = (from_node.0 * mult, from_node.1 * mult);

                    let candidate_one = (pos.0 + from_node.0, pos.1 + from_node.1);
                    let candidate_two = (node.0 + from_pos.0, node.1 + from_pos.1);

                    if candidate_one.0 >= 0 && candidate_one.0 < height as i64
                        && candidate_one.1 >= 0 && candidate_one.1 < width as i64 {
                            antinodes.insert((candidate_one.0 as usize, candidate_one.1 as usize));
                            found = true;
                        }
                    if candidate_two.0 >= 0 && candidate_two.0 < height as i64
                        && candidate_two.1 >= 0 && candidate_two.1 < width as i64 {
                            antinodes.insert((candidate_two.0 as usize, candidate_two.1 as usize));
                            found = true;
                        }

                    if !found {
                        break;
                    }
                }

            }

            nodes.insert((pos.0 as usize, pos.1 as usize));
        }
    }

    antinodes
}

fn distance(a: (usize, usize), b: (usize, usize)) -> i64 {
    let a = (a.0 as f64, a.1 as f64);
    let b = (b.0 as f64, b.1 as f64);
    let real_distance = f64::sqrt((b.0 - a.0).powi(2) + (b.1 - a.1).powi(2));
    real_distance as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn validate_part_one() {
        assert_eq!(part_one(INPUT), 14);
    }

    #[test]
    fn validate_part_two() {
        assert_eq!(part_two(INPUT), 34);
    }
}
