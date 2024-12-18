use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn run() {
    let path = Path::new("data/four");
    let mut file = File::open(&path).expect("Need input file");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("Need to read input");

    let res = search(&input);
    println!("XMAS count: {}", res);
}

fn search(input: &str) -> u64 {
    let lines: Vec<&str> = input.split('\n').collect();

    let mut score = 0;
    for (i, row) in lines.iter().enumerate() {
        for (j, char) in row.chars().enumerate() {
            if char == 'X' {
                score += deep_search(&lines, (i, j));
            }
        }
    }
    score
}

fn deep_search(input: &Vec<&str>, pos: (usize, usize)) -> u64 {
    let mut found: u64 = 0;
    let line = input[pos.0];
    // Horizontal to the right
    if pos.1 <= line.len() - 4 && line[pos.1..(pos.1 + 4)] == *"XMAS" {
        found += 1;
    }
    // To the left
    if pos.1 >= 3 && line[(pos.1 - 3)..(pos.1 + 1)] == *"SAMX" {
        found += 1
    }
    // The next two are probably faulty
    // Up
    if pos.0 >= 3
        && input[pos.0 - 1].chars().nth(pos.1).unwrap_or('\0') == 'M'
        && input[pos.0 - 2].chars().nth(pos.1).unwrap_or('\0') == 'A'
        && input[pos.0 - 3].chars().nth(pos.1).unwrap_or('\0') == 'S'
    {
        found += 1
    }
    // Down
    if pos.0 <= (input.len() - 3)
        && input[pos.0 + 1].chars().nth(pos.1).unwrap_or('\0') == 'M'
        && input[pos.0 + 2].chars().nth(pos.1).unwrap_or('\0') == 'A'
        && input[pos.0 + 3].chars().nth(pos.1).unwrap_or('\0') == 'S'
    {
        found += 1
    }
    found + search_diagonals(input, pos)
}

fn search_diagonals(input: &Vec<&str>, pos: (usize, usize)) -> u64 {
    let mut found: u64 = 0;

    // Up right
    if pos.0 >= 3 && pos.1 < (input[0].len() - 3) {
        if input[pos.0 - 1].chars().nth(pos.1 + 1).unwrap_or('\0') == 'M'
            && input[pos.0 - 2].chars().nth(pos.1 + 2).unwrap_or('\0') == 'A'
            && input[pos.0 - 3].chars().nth(pos.1 + 3).unwrap_or('\0') == 'S'
        {
            found += 1;
        }
    }

    // Up left
    if pos.0 >= 3 && pos.1 >= 3 {
        if input[pos.0 - 1].chars().nth(pos.1 - 1).unwrap_or('\0') == 'M'
            && input[pos.0 - 2].chars().nth(pos.1 - 2).unwrap_or('\0') == 'A'
            && input[pos.0 - 3].chars().nth(pos.1 - 3).unwrap_or('\0') == 'S'
        {
            found += 1;
        }
    }

    // Down left
    if pos.0 < input.len() - 3 && pos.1 >= 3 {
        if input[pos.0 + 1].chars().nth(pos.1 - 1).unwrap_or('\0') == 'M'
            && input[pos.0 + 2].chars().nth(pos.1 - 2).unwrap_or('\0') == 'A'
            && input[pos.0 + 3].chars().nth(pos.1 - 3).unwrap_or('\0') == 'S'
        {
            found += 1;
        }
    }

    // Down right
    if pos.0 < input.len() - 3 && pos.1 <= (input[0].len() - 3) {
        if input[pos.0 + 1].chars().nth(pos.1 + 1).unwrap_or('\0') == 'M'
            && input[pos.0 + 2].chars().nth(pos.1 + 2).unwrap_or('\0') == 'A'
            && input[pos.0 + 3].chars().nth(pos.1 + 3).unwrap_or('\0') == 'S'
        {
            found += 1;
        }
    }

    found
}

fn cross_mas_search(input: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_search() {
        assert_eq!(search(&INPUT), 18);
    }

    #[test]
    fn test_cross() {
        assert_eq!(cross_mas_search(&INPUT), 9);
    }
}
