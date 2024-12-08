use std::fs::read_to_string;

use rust::get_lines;

fn main() {
    let input = read_to_string("input.txt");

    if let Ok(input) = input {
        let lines = get_lines(input);

        let x_positions = lines
            .iter()
            .enumerate()
            .map(|(idx, letters)| {
                let x = letters.join("");
                x.match_indices('X')
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

        let xmas_forward = |x: &usize, y: &usize| {
            word_at(vec![
                &letter_at(x, &(y + 0)),
                &letter_at(x, &(y + 1)),
                &letter_at(x, &(y + 2)),
                &letter_at(x, &(y + 3)),
            ])
        };

        let xmas_backward = |x: &usize, y: &usize| {
            if *y >= 3 {
                word_at(vec![
                    &letter_at(x, &(y - 0)),
                    &letter_at(x, &(y - 1)),
                    &letter_at(x, &(y - 2)),
                    &letter_at(x, &(y - 3)),
                ])
            } else {
                String::new()
            }
        };

        let xmas_down = |x: &usize, y: &usize| {
            word_at(vec![
                &letter_at(&(x + 0), y),
                &letter_at(&(x + 1), y),
                &letter_at(&(x + 2), y),
                &letter_at(&(x + 3), y),
            ])
        };

        let xmas_up = |x: &usize, y: &usize| {
            if *x >= 3 {
                word_at(vec![
                    &letter_at(&(x - 0), y),
                    &letter_at(&(x - 1), y),
                    &letter_at(&(x - 2), y),
                    &letter_at(&(x - 3), y),
                ])
            } else {
                String::new()
            }
        };

        let xmas_diag_up_rt = |x: &usize, y: &usize| {
            if *y >= 3 {
                word_at(vec![
                    &letter_at(&(x + 0), &(y - 0)),
                    &letter_at(&(x + 1), &(y - 1)),
                    &letter_at(&(x + 2), &(y - 2)),
                    &letter_at(&(x + 3), &(y - 3)),
                ])
            } else {
                String::new()
            }
        };

        let xmas_diag_down_rt = |x: &usize, y: &usize| {
            word_at(vec![
                &letter_at(&(x + 0), &(y + 0)),
                &letter_at(&(x + 1), &(y + 1)),
                &letter_at(&(x + 2), &(y + 2)),
                &letter_at(&(x + 3), &(y + 3)),
            ])
        };

        let xmas_diag_up_lt = |x: &usize, y: &usize| {
            if *x >= 3 && *y >= 3 {
                word_at(vec![
                    &letter_at(&(x - 0), &(y - 0)),
                    &letter_at(&(x - 1), &(y - 1)),
                    &letter_at(&(x - 2), &(y - 2)),
                    &letter_at(&(x - 3), &(y - 3)),
                ])
            } else {
                String::new()
            }
        };

        let xmas_diag_down_lt = |x: &usize, y: &usize| {
            if *x >= 3 {
                word_at(vec![
                    &letter_at(&(x - 0), &(y + 0)),
                    &letter_at(&(x - 1), &(y + 1)),
                    &letter_at(&(x - 2), &(y + 2)),
                    &letter_at(&(x - 3), &(y + 3)),
                ])
            } else {
                String::new()
            }
        };

        let solution = x_positions
            .iter()
            .fold(vec![], |mut acc, (x, y)| {
                acc.extend(vec![
                    xmas_forward(x, y),
                    xmas_backward(x, y),
                    xmas_down(x, y),
                    xmas_up(x, y),
                    xmas_diag_up_rt(x, y),
                    xmas_diag_down_rt(x, y),
                    xmas_diag_up_lt(x, y),
                    xmas_diag_down_lt(x, y),
                ]);
                acc
            })
            .into_iter()
            .filter(|it| it == "XMAS")
            .collect::<Vec<String>>()
            .len();

        println!("{:?}", solution);
    }
}
