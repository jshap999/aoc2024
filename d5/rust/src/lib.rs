pub fn get_lines(input: String) -> Vec<String> {
    input.lines().map(String::from).collect::<Vec<String>>()
}
