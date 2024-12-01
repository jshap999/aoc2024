use std::fs::read_to_string;

use rust::{get_lines, get_sorted_list};

fn main() {
    let input = read_to_string("input.txt");
    if let Ok(input) = input {
        let lines = get_lines(input);

        let left_list = get_sorted_list(&lines, 0b0);
        let right_list = get_sorted_list(&lines, 0b1);

        let solution = left_list
            .iter()
            .zip(&right_list)
            .map(|(left, right)| (left - right).abs())
            .sum::<i64>();

        println!("{}", solution);
    }
}
