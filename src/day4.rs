use common;
use std::collections::HashSet;
use std::iter::FromIterator;
use itertools::Itertools;


fn sort_string(s : String) -> String {
    s
    .chars()
    .sorted()
    .iter()
    .collect::<String>()
}


fn is_valid(password: String, is_part1 : bool) -> bool {
    let words = password 
        .trim()
        .split_whitespace()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let words_len = words.clone().len();
    let unique_words : HashSet<String> = HashSet::from_iter(
        words
        .iter()
        .cloned()
        .map(|x| if is_part1 {x} else {sort_string(x)})
    );
    words_len == unique_words.len()
}


fn worker(passwords: String, is_part1: bool) -> usize {
    passwords.split('\n').map(|line| {
        if is_valid(line.to_string(), is_part1) {1} else {0}
    }).sum()
}


#[allow(dead_code)]
pub fn main() {
    let input = common::read_file("data/day4.txt");
    println!("day 4 - 1 {}", worker(input.clone(), true));
    println!("day 4 - 2 {}", worker(input, false));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        for &(sample, valid) in [
            ("aa bb cc dd ee", true),
            ("aa bb cc dd aa", false),
            ("aa bb cc dd aaa", true)
        ].iter() {
            assert_eq!(is_valid(sample.to_string(), true), valid);
        }
    }

    #[test]
    fn part2() {
        for &(sample, valid) in [
            ("abcde fghij", true),
            ("abcde xyz ecdab", false),
            ("a ab abc abd abf abj", true),
            ("iiii oiii ooii oooi oooo", true),
            ("oiii ioii iioi iiio", false)
        ].iter() {
            assert_eq!(is_valid(sample.to_string(), false), valid);
        }
    }

    #[test]
    fn solution1() {
        let input = common::read_file("data/day4.txt");
        assert_eq!(worker(input.clone(), true), 337);
    }

    #[test]
    fn solution2() {
        let input = common::read_file("data/day4.txt");
        assert_eq!(worker(input.clone(), false), 231);
    }
}
