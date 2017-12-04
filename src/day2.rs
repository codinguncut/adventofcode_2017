use common;
use itertools::Itertools;
use std;


fn part1(input: String) -> u32 {
    input.split('\n').map(|line| {
        let (min, max) : (u32, u32) = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .fold(None, |state, n| match state {
                None => Some((n, n)),
                Some((min, max)) =>
                    Some((std::cmp::min(min, n),
                          std::cmp::max(max, n)))
            })
            .unwrap();
        max - min
    }).sum()
}


fn part2(input: String) -> u32 {
    input.split('\n').map(|line| {
        let nums = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<u32>>();
        // NOTE: assuming only one match per line
        let res : u32 = nums.iter().tuple_combinations().map(|(&a, &b)| {
            if a % b == 0 {
                a / b as u32
            } else if b % a == 0 {
                b / a as u32
            } else {
                0
            }
        }).sum();
        res
    }).sum()
}


#[allow(dead_code)]
pub fn main() {
    let input = common::read_file("data/day2.txt");
    println!("day 2 - 1 {}", part1(input.to_string()));
    println!("day 2 - 2 {}", part2(input.to_string()));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let sample = "5 1 9 5
    7 5 3
    2 4 6 8".to_string();
        assert_eq!(part1(sample), 18);
    }

    #[test]
    fn test2() {
        let sample2 = "5 9 2 8
    9 4 7 3
    3 8 6 5".to_string();
        assert_eq!(part2(sample2), 9);
    }

    #[test]
    fn solution1() {
        let input = common::read_file("data/day2.txt");
        assert_eq!(part1(input.to_string()), 30994);
    }

    #[test]
    fn solution2() {
        let input = common::read_file("data/day2.txt");
        assert_eq!(part2(input.to_string()), 233);
    }
}
