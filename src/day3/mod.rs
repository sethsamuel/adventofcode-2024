use crate::file::read_file;
use regex::Regex;

type Multiplicand = usize;
type Multiplicands = Vec<(Multiplicand, Multiplicand)>;

pub fn parse_file(text: &str) -> Multiplicands {
    //
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(text)
        .map(|c| c.extract().1.map(|v| v.parse().unwrap()))
        .map(|v: [usize; 2]| (v[0], v[1]))
        .collect()
}

pub fn sum_and_multiply(values: Multiplicands) -> usize {
    values.iter().fold(0, |acc, v| acc + v.0 * v.1)
}

#[allow(dead_code)]
pub fn part1() {
    let input = read_file(module_path!());
    println!("{}", sum_and_multiply(parse_file(input.as_str())));
}

#[allow(dead_code)]
pub fn part2() {
    let _input = read_file(module_path!());
    // let reports = parse_file(input.as_str());
    // println!("{}", find_safe_count(reports, true));
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_STR: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn test_parse_file() {
        assert_eq!(parse_file(TEST_STR), vec![(2, 4), (5, 5), (11, 8), (8, 5)])
    }

    #[test]
    fn test_sum_and_multiply() {
        assert_eq!(sum_and_multiply(parse_file(TEST_STR)), 161)
    }
}
