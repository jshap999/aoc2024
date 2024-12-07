use regex_lite::Regex;
use std::fs::read_to_string;

use rust::get_lines;

fn main() {
    let input = read_to_string("sample2.txt");

    if let Ok(input) = input {
        let line = get_lines(input)
            .join("")
            .replace("don't()", "\n::")
            .replace("do()", "\n");

        let lines = line
            .split("\n")
            .filter(|&it| !it.starts_with("::"))
            .collect::<Vec<&str>>();

        let re = Regex::new(r"(mul\([0-9]{1,3},[0-9]{1,3}\))").unwrap();

        let solution: Vec<Vec<i64>> = lines
            .iter()
            .map(|&line| {
                re.captures_iter(line)
                    .map(|cap| {
                        let x = &cap[1];
                        x.to_string()
                            .replace("mul(", "")
                            .replace(")", "")
                            .split(',')
                            .flat_map(str::parse::<i64>)
                            .product::<i64>()
                    })
                    .collect()
            })
            .collect::<_>();

        println!("{:?}", solution.into_iter().flatten().sum::<i64>());
    }
}
