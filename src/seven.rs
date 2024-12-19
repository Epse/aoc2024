pub fn run() {
    let input = include_str!("../data/seven");
    println!("Part one: {} sum of possible", part_one(input));
    println!("Part two: {}", part_two(input));
}

struct Equation {
    pub result: i64,
    pub args: Vec<i64>,
}

fn part_one(input: &str) -> i64 {
    parse(input)
        .iter()
        .filter(|x| is_possible(&x))
        .map(|x| x.result)
        .sum()
}

fn part_two(input: &str) -> i64 {
    parse(input)
        .iter()
        .filter(|x| {
            let (head, tail) = x.args.split_first().unwrap();
            is_possible_with_concatenation(x.result, *head, tail)
        })
        .map(|x| x.result)
        .sum()
}

fn parse(input: &str) -> Vec<Equation> {
    input
        .lines()
        .filter_map(|line| line.split_once(": "))
        .filter_map(|(result, args)| {
            let result = result.parse::<i64>().ok()?;
            let args = args
                .split(" ")
                .filter_map(|x| x.parse::<i64>().ok())
                .collect();

            Some(Equation { result, args })
        })
        .collect()
}

fn is_possible_with_concatenation(result: i64, current: i64, numbers: &[i64]) -> bool {
    if numbers.is_empty() {
        return result == current;
    }

    if current > result {
        return false;
    }

    let (head, tail) = numbers.split_first().unwrap();
    is_possible_with_concatenation(result, current * head , tail)
        || is_possible_with_concatenation(result, current + head, tail)
        || is_possible_with_concatenation(result, concat(current, *head), tail)
}

fn concat(head: i64, tail: i64) -> i64 {
    let mut magnitude = 1;
    while magnitude <= tail {
        magnitude *= 10;
    }
    head * magnitude + tail
}

fn is_possible(eq: &Equation) -> bool {
    let max_iterations = 2usize.checked_pow((eq.args.len() - 1) as u32).unwrap();

    (0..max_iterations)
        .any(|x| correct(eq, x))
}

fn correct(eq: &Equation, ops: usize) -> bool {
    let mut result = eq.args[0];
    for i in 0..eq.args.len()-1 {
        let mask: usize = 1 << i;
        match ops & mask > 0 {
            false => {
                result += eq.args[i + 1];
            },
            true => result *= eq.args[i + 1],
        };

        // Tiny short-circuit optimisation
        if result > eq.result {
            return false;
        }
    }
    result == eq.result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn validate_is_possible() {
        assert_eq!(
            is_possible(&Equation {
                result: 190,
                args: vec![10, 19]
            }),
            true
        );
        assert_eq!(
            is_possible(&Equation {
                result: 7290,
                args: vec![6, 8, 6, 15]
            }),
            false
        );
    }

    #[test]
    fn test_correct() {
        assert_eq!(correct(&Equation{result: 190, args: vec![10, 19]}, 0b0001usize), true);
        assert_eq!(correct(
            &Equation{result: 3267, args: vec![81, 40, 27]},
            0b0011usize,
        ), false);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), 3749);
    }

    #[test]
    fn test_concatenation() {
        assert_eq!(part_two(INPUT), 11387);
    }
}
