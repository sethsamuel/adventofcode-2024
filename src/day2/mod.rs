use crate::file::read_file;

type Level = usize;
type Report = Vec<Level>;
type Reports = Vec<Report>;

pub fn parse_file(text: &str) -> Reports {
    text.split('\n').map(parse_line).collect()
}

pub fn parse_line(line: &str) -> Report {
    line.split_whitespace()
        .map(|d| d.parse::<usize>().unwrap())
        .collect()
}

pub fn is_safe(report: &Report) -> bool {
    if report.len() < 2 {
        return true;
    }
    let mut last = &report[0];
    let is_increasing = report[1].gt(&report[0]);
    for value in report.iter().skip(1) {
        if !(1..=3).contains(&value.abs_diff(*last)) {
            return false;
        }
        if is_increasing && value.lt(last) {
            return false;
        }
        if !is_increasing && value.gt(last) {
            return false;
        }

        last = value
    }
    true
}

pub fn find_safe_count(reports: Reports) -> usize {
    reports.iter().filter(|r| is_safe(r)).count()
}

#[allow(dead_code)]
pub fn part1() {
    let input = read_file(module_path!());
    println!("{}", find_safe_count(parse_file(input.as_str())));
}

#[allow(dead_code)]
pub fn part2() {
    // let input = read_file(module_path!());
    // println!("{}", find_id_similarity(parse_file(input.as_str())));
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_STR: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("1  2"), vec![1, 2]);
        assert_eq!(parse_line(" 12  2"), vec![12, 2]);
        assert_eq!(parse_line(" 1   202 3"), vec![1, 202, 3]);
    }

    #[test]
    fn test_parse_file() {
        assert_eq!(
            parse_file("3 3 4\n1  2\n21 1 1  12"),
            vec![vec![3, 3, 4], vec![1, 2], vec![21, 1, 1, 12]]
        )
    }

    #[test]
    fn test_is_safe() {
        assert!(is_safe(&vec![]));
        assert!(is_safe(&vec![1]));
        assert!(is_safe(&vec![1, 2, 4]));
        assert!(!is_safe(&vec![1, 0, 1]));
        assert!(!is_safe(&vec![1, 101, 1]));
    }

    #[test]
    fn test_find_safe_count() {
        assert_eq!(find_safe_count(parse_file(TEST_STR)), 2)
    }

    #[test]
    fn test_find_id_similarity() {
        // assert_eq!(find_id_similarity(parse_file(TEST_STR)), 31)
    }
}
