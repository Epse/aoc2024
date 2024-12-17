use std::{collections::HashMap, path::Path};
use crate::parse::parse_two_i32_lists;


pub fn run() {
    let input = parse_two_i32_lists(Path::new("data/one_one")).unwrap();
    let delta = compute_delta(&input.0, &input.1).unwrap();
    println!("Delta is {}", delta);
    println!("Similarity is {}", compute_similarity(&input.0, &input.1).unwrap_or(0));
}

fn compute_delta(one: &Vec<i32>, two: &Vec<i32>) -> Result<i32, String> {
    if one.len() != two.len() {
        return Err(String::from("Lists do not have the same length!"));
    }

    let mut sorted_one = one.clone();
    sorted_one.sort();
    let mut sorted_two = two.clone();
    sorted_two.sort();


    return Ok(sorted_one.iter()
        .zip(sorted_two.iter())
        .map(|(a, b)|(a - b).abs())
        .sum());
}

fn compute_similarity(one: &Vec<i32>, two: &Vec<i32>) -> Result<i32, String> {
    let mut counts: HashMap<i32, i32> = HashMap::new();
    two.iter()
       .for_each(|x| {
           counts.insert(*x, counts.get(x).unwrap_or(&0) + 1);
       });

    Ok(one.clone()
       .iter()
       .map(|x| x * counts.get(x).unwrap_or(&0))
       .sum())
}
