use regex_lite::Regex;
use std::fs::read_to_string;

use rust::get_lines;

fn main() {
    let input = read_to_string("sample1.txt");

    if let Ok(input) = input {
        let line = get_lines(input).join("");

        let re = Regex::new(r"(mul\([0-9]{1,3},[0-9]{1,3}\))").unwrap();

        let solution: Vec<i64> = re
            .captures_iter(&line)
            .map(|cap| {
                let x = &cap[1];
                x.to_string()
                    .replace("mul(", "")
                    .replace(")", "")
                    .split(',')
                    .flat_map(str::parse::<i64>)
                    .product::<i64>()
            })
            .collect::<Vec<_>>();

        println!("{:?}", solution.iter().sum::<i64>());
    }
}
