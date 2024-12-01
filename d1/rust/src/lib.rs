pub fn get_lines(input: String) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(str::trim)
        .map(String::from)
        .map(|it| {
            it.split(' ')
                .filter(|&it| it != "")
                .map(str::parse::<i64>)
                .map(|it| it.unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn get_sorted_list(lines: &Vec<Vec<i64>>, idx: usize) -> Vec<i64> {
    let mut list = lines.iter().map(|it| it[idx]).collect::<Vec<i64>>();
    list.sort();
    list
}
