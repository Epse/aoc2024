use super::rule::Rule;

pub fn reorder_pageset(pageset: &Vec<i64>, rules: &Vec<Rule>) -> Vec<i64> {
    let mut choices = pageset.clone();
    let mut result = Vec::new();

    for _ in 0..pageset.len() {
        for (idx, ele) in choices.iter().enumerate() {
            if rules.iter()
                .filter(|x| x.last == *ele)
                .any(|x| !result.contains(&x.first) && pageset.contains(&x.first)) {
                    continue;
                }
            result.push(*ele);
            choices.remove(idx);
            break;
        }
    }

    result
}
