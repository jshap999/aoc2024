use std::{cmp::Ordering, collections::HashMap, fs::read_to_string};

use rust::get_lines;

fn main() {
    let input = read_to_string("input.txt");

    if let Ok(input) = input {
        let lines = get_lines(input);

        let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut page_numers: Vec<Vec<i32>> = Vec::new();

        lines.iter().for_each(|line| {
            if line.contains("|") {
                let x = &line
                    .split("|")
                    .map(str::parse::<i32>)
                    .map(|it| it.unwrap())
                    .collect::<Vec<i32>>();

                if rules.contains_key(&x[0]) {
                    let mut vals = rules.get(&x[0]).unwrap().clone();
                    vals.push(x[1]);
                    rules.insert(x[0], vals.clone());
                } else {
                    rules.insert(x[0], vec![x[1]]);
                }
            } else if line.contains(",") {
                let x = &line
                    .split(",")
                    .map(str::parse::<i32>)
                    .map(|it| it.unwrap())
                    .collect::<Vec<i32>>();

                page_numers.push(x.clone());
            }
        });

        let solution = page_numers
            .into_iter()
            .map(|mut list| {
                let list_hold = list.clone();
                list.sort_by(|a, b| {
                    let rules_a = rules.get(a);
                    let rules_b = rules.get(b);
                    if rules_a.is_some() && rules.get(a).unwrap().contains(b) {
                        Ordering::Less
                    } else if rules_b.is_some() && rules.get(b).unwrap().contains(a) {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                });

                if list_hold == list {
                    list[list.len().div_ceil(2) - 1]
                } else {
                    0
                }
            })
            .sum::<i32>();

        println!("{:?}", solution);
    }
}
