use std::{fs::File, io::Read, path::Path};
use regex::Regex;

pub fn run() {
    let path = Path::new("data/three");
    let mut file = File::open(&path).expect("Need input file");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("Need to read input");

    println!("Mult: {}", do_mul(&input));
    println!("Conditional (lazy): {}", do_conditional(&input));
}

fn do_mul(input: &str) -> i64 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").expect("Need a regex");
    re.captures_iter(input).map(|c| c.extract()).map(|(_, [x, y])| {
        x.parse::<i64>().expect("needs numbershaped")
            * y.parse::<i64>().expect("Needs numbershaped")
    }).sum()
}

fn do_conditional(input: &str) -> i64 {
    let mut enabled = true;
    let mut sum: i64 = 0;
    for elem in input.split("do") {
        if elem.starts_with("n't") {
            enabled = false
        }
        if elem.starts_with("()") {
            enabled = true
        }
        if enabled {
            sum += do_mul(&elem);
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_input() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(do_mul(input), 161);
    }

    #[test]
    fn conditional_example() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(do_conditional(&input), 48);
    }
}
