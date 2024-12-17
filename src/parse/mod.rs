use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::path::Path;

// In theory making this output iters somehow might speed it up marginally, but oh well
pub fn parse_two_i32_lists(path: &Path) -> Result<(Vec<i32>, Vec<i32>), String> {
    let file = File::open(path).expect("The file really should exist...");
    let lines: (Vec<i32>, Vec<i32>) = io::BufReader::new(file).lines()
                                        .filter_map(|x| {
                                            if x.is_err() {
                                                return None;
                                            }

                                            return split_line_to_num(&x.unwrap());
                                        })
        .unzip();
    Ok(lines)
}

fn parse_matrix<T>(path: &Path) -> Result<Vec<Vec<T>>, String>
    where T: std::str::FromStr
{
    let file = File::open(path).expect("Requires datafile");
    let lines = io::BufReader::new(file)
        .lines()
        .filter_map(|line| {
            if line.is_err() {
                None
            } else {
                Some(
                    // Unwrap not needed
                    line.unwrap().split(" ")
                        .filter_map(|x| x.parse::<T>().ok())
                        .collect::<Vec<T>>()
                )
            }
        })
        .collect::<Vec<Vec<T>>>();
    Ok(lines)
}

fn split_line_to_num(line: &String) -> Option<(i32, i32)> {
    let split = line.split(" ").collect::<Vec<&str>>();
    if split.len() < 2 {
        None
    } else {
        let first = split[0].parse::<i32>().ok()?;
        let second = split[split.len() - 1].parse::<i32>().ok()?;
        Some((first, second))
    }
}
