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
    println!("X-MAS count: {}", cross_mas_search(&input));
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
    // Plus one to deal with newlines
    let line_length = input.find('\n').expect("Needs at least one newline lol") + 1;
    let chars = input.chars().collect::<Vec<char>>();
    let lines = input.chars().filter(|x| *x == '\n').count();

    let mut crosses: u64 = 0;

    // Approach: find an A, check for m and s
    for i in 0..chars.len() {
        let row: usize = i / line_length;
        if row == 0 {
            continue; // line 0 cannot have a cross, obvs
        }
        if row == lines - 1 {
            break; // Last line can ofc also not start a cross
        }
        let col = i % line_length;
        if col == 0 || col == line_length - 2 { // Minus 2 to deal with \n
            continue; // First and last col also cannot have a center
        }

        if chars[i] != 'A' {
            continue;
        }

        // Check if we have a cross
        let top_left = chars[i - line_length - 1];
        let top_right = chars[i - line_length + 1];
        let bottom_left = chars[i + line_length - 1];
        let bottom_right = chars[i + line_length + 1];

        if !(top_left == 'M' && bottom_right == 'S' || top_left == 'S' && bottom_right == 'M') {
            continue;
        }

        if top_right == 'M' && bottom_left == 'S' || top_right == 'S' && bottom_left == 'M' {
            crosses += 1;
        }
    }
    crosses
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
MXMXAXMASX
";

    #[test]
    fn test_search() {
        assert_eq!(search(&INPUT), 18);
    }

    #[test]
    fn test_cross() {
        assert_eq!(cross_mas_search(&INPUT), 9);
    }
}
