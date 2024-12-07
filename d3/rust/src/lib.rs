pub fn get_lines(input: String) -> Vec<String> {
    input.lines().map(String::from).collect::<Vec<_>>()
}

pub fn get_sorted_list(lines: &Vec<Vec<i64>>, idx: usize) -> Vec<i64> {
    let mut list = lines.iter().map(|it| it[idx]).collect::<Vec<i64>>();
    list.sort();
    list
}

pub fn is_decending(reports: &Vec<i64>) -> bool {
    reports
        .windows(2)
        .fold(true, |acc, window| acc && window[0] > window[1])
}

pub fn is_decending_permissive(reports: &Vec<i64>) -> bool {
    let mut idx = 0;
    let mut safe = is_decending(reports) && is_dist_under_four(reports);

    while idx < reports.len() && safe == false {
        let mut dampened_reports = reports.clone();
        dampened_reports.remove(idx);
        idx += 1;
        safe = is_decending(&dampened_reports) && is_dist_under_four(&dampened_reports);
    }

    safe
}

pub fn is_ascending(reports: &Vec<i64>) -> bool {
    reports
        .windows(2)
        .fold(true, |acc, window| acc && window[0] < window[1])
}

pub fn is_ascending_permissive(reports: &Vec<i64>) -> bool {
    let mut idx = 0;
    let mut safe = is_ascending(reports) && is_dist_under_four(reports);

    while idx < reports.len() && safe == false {
        let mut dampended_reports = reports.clone();
        dampended_reports.remove(idx);
        idx += 1;
        safe = is_ascending(&dampended_reports) && is_dist_under_four(&dampended_reports);
    }

    safe
}

pub fn is_dist_under_four(reports: &Vec<i64>) -> bool {
    reports.windows(2).fold(true, |acc, window| {
        acc && (window[0].abs_diff(window[1]) < 4)
    })
}
