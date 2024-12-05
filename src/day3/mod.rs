use crate::file::read_file;

enum ParseState {
    Start,
    M,
    U,
    L,
    Digit,
    Comma,
    LeftParen,
}

type Multiplicand = usize;
type Multiplicands = Vec<(Multiplicand, Multiplicand)>;

fn reset(buffer: &mut usize, state: &mut ParseState) {
    *buffer = 0;
    *state = ParseState::Start
}

pub fn parse_file(text: &str) -> Multiplicands {
    //
    // let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    // re.captures_iter(text)
    //     .map(|c| c.extract().1.map(|v| v.parse().unwrap()))
    //     .map(|v: [usize; 2]| (v[0], v[1]))
    //     .collect()
    let mut state = ParseState::Start;
    let mut multiplicands = vec![];

    let mut buffer = 0;
    let mut left: Multiplicand = 0;

    for c in text.chars() {
        match state {
            ParseState::Start => match c {
                'm' => state = ParseState::M,
                _ => reset(&mut buffer, &mut state),
            },
            ParseState::M => match c {
                'u' => state = ParseState::U,
                _ => reset(&mut buffer, &mut state),
            },
            ParseState::U => match c {
                'l' => state = ParseState::L,
                _ => reset(&mut buffer, &mut state),
            },
            ParseState::L => match c {
                '(' => state = ParseState::LeftParen,
                _ => reset(&mut buffer, &mut state),
            },
            ParseState::LeftParen => match c.is_ascii_digit() {
                true => {
                    buffer = buffer * 10 + c.to_string().parse::<Multiplicand>().unwrap();
                    state = ParseState::Digit;
                }
                false => reset(&mut buffer, &mut state),
            },
            ParseState::Digit => match c.is_ascii_digit() {
                true => {
                    buffer = buffer * 10 + c.to_string().parse::<Multiplicand>().unwrap();
                    state = ParseState::Digit
                }
                false => match c {
                    ',' => {
                        left = buffer;
                        buffer = 0;
                        state = ParseState::Comma
                    }
                    ')' => {
                        multiplicands.push((left, buffer));
                        reset(&mut buffer, &mut state);
                    }
                    _ => reset(&mut buffer, &mut state),
                },
            },
            ParseState::Comma => match c.is_ascii_digit() {
                true => {
                    buffer = buffer * 10 + c.to_string().parse::<Multiplicand>().unwrap();
                    state = ParseState::Digit;
                }
                false => reset(&mut buffer, &mut state),
            },
        }
    }

    multiplicands
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
        assert_eq!(parse_file("mul(1,23)"), vec![(1, 23)]);
        assert_eq!(parse_file(TEST_STR), vec![(2, 4), (5, 5), (11, 8), (8, 5)])
    }

    #[test]
    fn test_sum_and_multiply() {
        assert_eq!(sum_and_multiply(parse_file(TEST_STR)), 161)
    }
}
