pub fn get_lines(input: String) -> Vec<Vec<String>> {
    input
        .lines()
        .map(String::from)
        .map(|it| it.chars().map(String::from).collect())
        .collect::<Vec<Vec<_>>>()
}
