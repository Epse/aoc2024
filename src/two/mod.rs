use crate::parse::parse_matrix;
use std::path::Path;

pub fn run(debug: bool) {
    // A row is a report
    let data = if debug {
        vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ]
    } else {
        parse_matrix::<i32>(&Path::new("data/two")).expect("Need data bruv")
    };
    /*
    let data = ;
    */
    let safe_count: i32 = data
        .iter()
        .map(|x| check_record(x).is_none())
        .map(|x| if x { 1 } else { 0 })
        .sum();
    println!("{} records safe", safe_count);

    let filtered_safe: i32 = data
        .iter()
        .map(|x| check_record_filtering(x))
        .map(|x| if x { 1 } else { 0 })
        .sum();
    println!("{} records safe", filtered_safe);
}

fn check_record_filtering(record: &Vec<i32>) -> bool {
    let pure = check_record(record);
    if pure.is_none() {
        return true;
    }

    // Exhaustively check all single-removals to check for safeness
    for i in 0..(record.len() - 1) {
        if check_record(&remove_idx(i, &record)).is_none() {
            return true;
        }
    }
    return false;
}

fn remove_idx(idx: usize, v: &Vec<i32>) -> Vec<i32> {
    v.iter()
        .enumerate()
        .filter_map(|(i, ele)| if idx == i { None } else { Some(*ele) })
        .collect::<Vec<i32>>()
}

fn check_record(record: &Vec<i32>) -> Option<usize> {
    let mut increasing = true;

    for (i, el) in record.iter().enumerate() {
        if i == 0 {
            continue;
        }

        if *el == record[i - 1] {
            return Some(i);
        }

        if i == 1 {
            if *el < record[i - 1] {
                increasing = false
            }
        }

        if increasing {
            if *el < record[i - 1] || *el - record[i - 1] > 3 {
                return Some(i);
            }
        } else {
            if *el > record[i - 1] || record[i - 1] - *el > 3 {
                return Some(i);
            }
        }
    }

    // If we haven't bailed by now, we gud
    None
}
