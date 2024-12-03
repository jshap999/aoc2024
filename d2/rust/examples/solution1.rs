use std::fs::read_to_string;

use rust::{get_lines, is_ascending, is_decending, is_dist_under_four};

fn main() {
    let input = read_to_string("input.txt");

    if let Ok(input) = input {
        let lines = get_lines(input);

        let solution = lines
            .iter()
            .map(|reports| {
                is_dist_under_four(reports) && (is_ascending(reports) || is_decending(reports))
            })
            .filter(|&val| val == true)
            .count();

        println!("{:?}", solution);
    }
}
