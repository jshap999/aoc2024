use std::fs::read_to_string;

use rust::{
    get_lines, is_ascending_permissive, is_decending_permissive
};

fn main() {
    let input = read_to_string("input.txt");

    if let Ok(input) = input {
        let lines = get_lines(input);

        let report_safety = lines
            .iter()
            .map(|reports| {
                is_ascending_permissive(reports) || is_decending_permissive(reports)
            })
            .collect::<Vec<_>>();

        let solution = report_safety.iter().filter(|&val| *val == true).count();

        println!("{:?}", solution);
    }
}
