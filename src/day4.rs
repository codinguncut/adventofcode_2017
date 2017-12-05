use common;
use std::collections::HashSet;
use std::iter::FromIterator;
use itertools::Itertools;


/// sort string by character to create unique char fingerprint
fn sort_string(s : &str) -> String {
    s.chars()
        .sorted()
        .iter()
        .collect::<String>()
}


/// check if given passphrase is valid
/// # Arguments
/// `is_part1` toggles between the two parts of the problem
fn is_valid(password: &str, is_part1 : bool) -> bool {
    let words = password 
        .trim()
        .split_whitespace()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let words_len = words.len();
    let unique_words : HashSet<String> = HashSet::from_iter(
        words
        .iter()
        .cloned()
        .map(|x| if is_part1 {x} else {sort_string(&x)})
    );
    words_len == unique_words.len()
}


fn worker(passwords: &str, is_part1: bool) -> usize {
    passwords.split('\n').map(|line| {
        if is_valid(line, is_part1) {1} else {0}
    }).sum()
}


#[allow(dead_code)]
pub fn main() {
    let input = common::read_file("data/day4.txt");
    println!("day 4 - 1 {}", worker(&input, true));
    println!("day 4 - 2 {}", worker(&input, false));
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
            assert_eq!(is_valid(sample, true), valid);
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
            assert_eq!(is_valid(sample, false), valid);
        }
    }

    #[test]
    fn solution1() {
        let input = common::read_file("data/day4.txt");
        assert_eq!(worker(&input, true), 337);
    }

    #[test]
    fn solution2() {
        let input = common::read_file("data/day4.txt");
        assert_eq!(worker(&input, false), 231);
    }
}
