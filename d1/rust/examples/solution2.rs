use std::fs::read_to_string;

use rust::{get_lines, get_sorted_list};

fn main() {
    let input = read_to_string("input.txt");
    if let Ok(input) = input {
        let lines = get_lines(input);

        let left_list = get_sorted_list(&lines, 0b0);
        let right_list = get_sorted_list(&lines, 0b1);

        let solution = left_list
            .into_iter()
            .map(|it| it * (right_list.iter().filter(|&it2| *it2 == it).count() as i64))
            .sum::<i64>();

        println!("{:?}", solution);
    }
}
