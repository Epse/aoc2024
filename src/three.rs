use std::{fs::File, io::Read, path::Path};
use regex::Regex;

pub fn run() {
    let path = Path::new("data/three");
    let mut file = File::open(&path).expect("Need input file");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("Need to read input");

    println!("Mult: {}", do_mul(&input));
}

fn do_mul(input: &str) -> i64 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").expect("Need a regex");
    re.captures_iter(input).map(|c| c.extract()).map(|(_, [x, y])| {
        x.parse::<i64>().expect("needs numbershaped")
            * y.parse::<i64>().expect("Needs numbershaped")
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_input() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(do_mul(input), 161);
    }
}
