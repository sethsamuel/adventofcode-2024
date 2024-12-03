use crate::file::read_file;

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_STR: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_parse_file() {}

    #[test]
    fn test_find_id_distance() {}
}

type IdPair = (usize, usize);
type IdPairs = Vec<IdPair>;

pub fn parse_file(text: &str) -> IdPairs {
    unimplemented!();
}

pub fn parse_line(line: &str) -> IdPair {
    unimplemented!();
}

pub fn find_id_distance(pairs: IdPair) -> usize {
    unimplemented!();
}

pub fn part1() {}
