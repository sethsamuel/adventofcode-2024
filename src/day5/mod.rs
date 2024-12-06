use std::collections::{HashMap, HashSet};

use crate::file::read_file;

type Page = usize;
type Rule = (Page, Page);
type Rules = Vec<Rule>;
type Print = Vec<Page>;
type Prints = Vec<Print>;
type Requirements = HashMap<Page, Vec<Page>>;

pub fn parse_file(text: &str) -> (Rules, Prints) {
    let sections: Vec<&str> = text.split("\n\n").collect();
    let rules: Rules = sections[0]
        .split('\n')
        .map(|l| l.split('|').map(|d| d.parse::<Page>().unwrap()).collect())
        .map(|r: Vec<Page>| (r[0], r[1]))
        .collect();
    let prints: Prints = sections[1]
        .split('\n')
        .map(|l| {
            l.split(',')
                .map(|d| d.parse::<Page>().unwrap())
                .collect::<Vec<Page>>()
        })
        .collect();

    (rules, prints)
}

pub fn map_rules(rules: &Rules) -> Requirements {
    let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
    for rule in rules {
        let default = vec![];
        let mut requires = map.get(&rule.1).unwrap_or(&default).clone();
        requires.push(rule.0);
        map.insert(rule.1, requires.to_vec());
    }
    map
}

pub fn is_valid(print: &Print, requirements: &Requirements) -> bool {
    let mut seen: HashSet<Page> = HashSet::new();
    let mut forbidden: HashSet<Page> = HashSet::new();
    for page in print {
        if forbidden.contains(page) {
            return false;
        }
        seen.insert(*page);
        for requires in requirements.get(page).unwrap_or(&vec![]) {
            if !seen.contains(requires) {
                forbidden.insert(*requires);
            }
        }
    }

    true
}

pub fn get_middle(print: &Print) -> Page {
    print[(print.len() + 1) / 2 - 1]
}

pub fn sum_pages(prints: &Prints, requirements: &Requirements) -> usize {
    prints
        .iter()
        .filter(|p| is_valid(p, requirements))
        .map(get_middle)
        .sum()
}

#[allow(dead_code)]
pub fn part1() {
    let input = read_file(module_path!());
    let (rules, prints) = parse_file(input.as_str());
    let requires = map_rules(&rules);
    println!("{}", sum_pages(&prints, &requires));
}

#[allow(dead_code)]
pub fn part2() {
    let _input = read_file(module_path!());
    // println!("{}", count_x_mas(parse_file(input.as_str())));
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_STR: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_parse_file() {
        let (rules, prints) = parse_file(TEST_STR);
        let requires = map_rules(&rules);
        assert_eq!(requires.get(&13).unwrap().len(), 6);
        assert!(requires.get(&13).unwrap().contains(&75));
        assert!(requires.get(&13).unwrap().contains(&53));
        assert_eq!(prints.len(), 6);
        assert_eq!(prints[2].len(), 3);

        assert!(is_valid(&prints[0], &requires));
        assert!(is_valid(&prints[1], &requires));
        assert!(is_valid(&prints[2], &requires));
        assert!(!is_valid(&prints[3], &requires));
        assert!(!is_valid(&prints[4], &requires));
        assert!(!is_valid(&prints[5], &requires));
    }

    #[test]
    fn test_get_middle() {
        assert_eq!(get_middle(&vec![75, 47, 61, 53, 29]), 61);
        assert_eq!(get_middle(&vec![75]), 75);
    }

    #[test]
    fn test_sum_pages() {
        let (rules, prints) = parse_file(TEST_STR);
        let requires = map_rules(&rules);
        let sum = sum_pages(&prints, &requires);
        assert_eq!(sum, 143);
    }
}
