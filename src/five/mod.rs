use std::fs::File;
use std::io::Read;
use std::path::Path;

mod rule;
use rule::Rule;

pub fn run() {
    let path = Path::new("data/five");
    let mut file = File::open(&path).expect("Need input file");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("Need to read input");

    println!("Part one: {}", part_one(&input));
}

fn part_one(input: &str) -> i64 {
    let (rules, pagesets) = parse_input(input);
    let good_sets = pagesets
        .iter()
        .filter(|x| check_pageset(x, &rules));
    let middles = good_sets.map(|x| x.get((x.len() - 1) / 2).expect("Must have a middle"));
    middles.sum()
}

fn check_pageset(pageset: &Vec<i64>, rules: &Vec<Rule>) -> bool {
    let mut seen: Vec<i64> = Vec::new();

    for ele in pageset {
        if rules
            .iter()
            .filter(|rule| rule.last == *ele)
            .any(|rule| !seen.contains(&rule.first) && pageset.contains(&rule.first))
        {
            return false;
        }

        seen.push(*ele);
    }
    true
}

fn parse_input(input: &str) -> (Vec<Rule>, Vec<Vec<i64>>) {
    let chunks = input.split("\n\n").collect::<Vec<&str>>();
    (parse_rules(chunks[0]), parse_pagesets(chunks[1]))
}

fn parse_rules(text: &str) -> Vec<Rule> {
    text.lines()
        .filter_map(|x| x.split_once("|"))
        .map(|(x, y)| Rule {
            first: x.parse::<i64>().expect("Must be numeric"),
            last: y.parse::<i64>().expect("Must be numeric"),
        })
        .collect()
}

fn parse_pagesets(text: &str) -> Vec<Vec<i64>> {
    text.lines()
        .map(|line| {
            line.split(",")
                .map(|x| x.parse::<i64>().expect("Page must be numeric"))
                .collect::<Vec<i64>>()
        })
        .filter(|x| !x.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), 143);
    }

    #[test]
    fn test_parse_rules() {
        let input = "61|13
75|53
29|13
";
        let expected = vec![
            Rule {
                first: 61,
                last: 13,
            },
            Rule {
                first: 75,
                last: 53,
            },
            Rule {
                first: 29,
                last: 13,
            },
        ];
        let result = parse_rules(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_pagesets() {
        let input = "75,97,47,61,53
61,13,29
97,13,75,29,47
";
        let expected = vec![
            vec![75, 97, 47, 61, 53],
            vec![61, 13, 29],
            vec![97, 13, 75, 29, 47],
        ];
        assert_eq!(parse_pagesets(input), expected);
    }
}
