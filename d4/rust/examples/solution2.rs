use std::fs::read_to_string;

use rust::get_lines;

fn main() {
    let input = read_to_string("input.txt");

    if let Ok(input) = input {
        let lines = get_lines(input);

        let a_positions = lines
            .iter()
            .enumerate()
            .map(|(idx, letters)| {
                let x = letters.join("");
                x.match_indices('A')
                    .map(|(pos, _)| (idx, pos))
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<Vec<_>>();

        let letter_at = |x: &usize, y: &usize| {
            let row = if x < &lines.len() {
                lines.get(*x)
            } else {
                None
            };

            match row {
                Some(row) => {
                    if y < &row.len() {
                        String::from(row.get(*y).map_or("", |v| v))
                    } else {
                        String::new()
                    }
                }
                None => String::new(),
            }
        };

        let word_at = |letters: Vec<&String>| {
            letters
                .into_iter()
                .map(|c| c.to_owned())
                .collect::<Vec<String>>()
                .join("")
        };

        let xmas_diag_up_rt = |x: &usize, y: &usize| {
            if *x >= 1 && *y >= 1 {
                word_at(vec![
                    &letter_at(&(x - 1), &(y + 1)),
                    &letter_at(&(x + 0), &(y - 0)),
                    &letter_at(&(x + 1), &(y - 1)),
                ])
            } else {
                String::new()
            }
        };

        let xmas_diag_down_rt = |x: &usize, y: &usize| {
            if *x >= 1 && *y >= 1 {
                word_at(vec![
                    &letter_at(&(x - 1), &(y - 1)),
                    &letter_at(&(x + 0), &(y + 0)),
                    &letter_at(&(x + 1), &(y + 1)),
                ])
            } else {
                String::new()
            }
        };

        let xmas_diag_up_lt = |x: &usize, y: &usize| {
            if *x >= 1 && *y >= 1 {
                word_at(vec![
                    &letter_at(&(x + 1), &(y + 1)),
                    &letter_at(&(x - 0), &(y - 0)),
                    &letter_at(&(x - 1), &(y - 1)),
                ])
            } else {
                String::new()
            }
        };

        let xmas_diag_down_lt = |x: &usize, y: &usize| {
            if *x >= 1 && *y >= 1 {
                word_at(vec![
                    &letter_at(&(x + 1), &(y - 1)),
                    &letter_at(&(x - 0), &(y + 0)),
                    &letter_at(&(x - 1), &(y + 1)),
                ])
            } else {
                String::new()
            }
        };

        let solution = a_positions.iter().fold(vec![], |mut acc, (x, y)| {
            let x = vec![
                xmas_diag_up_rt(x, y),
                xmas_diag_down_rt(x, y),
                xmas_diag_up_lt(x, y),
                xmas_diag_down_lt(x, y),
            ];
            if x.into_iter()
                .filter(|it| it == "MAS")
                .collect::<Vec<String>>()
                .len()
                == 2
            {
                acc.push("XMAS")
            }
            acc
        });

        println!("{:?}", solution.len());
    }
}
