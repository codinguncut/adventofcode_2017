extern crate itertools;

use std::fs::File;
use std::io::prelude::*;
use itertools::Itertools;


/// read content of file 'fname' and return as String
fn read_file(fname: &str) -> String {
    let mut f = File::open(fname)
        .expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    return contents.trim().to_string();
}


fn advent1() {
    // get characters that are the same in 's' with distance 'offset'
    fn get_same(s: String, offset: usize) -> Vec<char> {
        let t = [&s[offset..], &s[..offset]].join("");
        return s.chars().zip(t.chars())
            .filter(|&(a, b)| a == b)
            .map(|(a, _)| a)
            .collect::<Vec<_>>();
    }

    // turn vector of characters into vector of numbers
    fn to_num(a: Vec<char>) -> Vec<u32> {
        return a
            .iter()
            .map(|c| c.to_digit(10)
            .unwrap())
            .collect::<Vec<_>>();
    }

    fn worker(s: String, offset: usize) -> u32 {
        return to_num(get_same(s, offset)).iter().sum();
    }

    for &(sample, valid) in [
        ("1122", 3),
        ("1111", 4),
        ("1234", 0),
        ("91212129", 9)
    ].iter() {
        assert_eq!(worker(sample.to_string(), 1), valid);
    }

    for &(sample, valid) in [
        ("1212", 6),
        ("1221", 0),
        ("123425", 4),
        ("123123", 12),
        ("12131415", 4),
    ].iter() {
        let offset = sample.chars().count() / 2 as usize;
        assert_eq!(worker(sample.to_string(), offset), valid);
    }

    let input = read_file("data/day1.txt");
    let offset = input.chars().count() / 2 as usize;
    println!("day 1 - 1 {}", worker(input.to_string(), 1));
    println!("day 1 - 2 {}", worker(input.to_string(), offset));
}


fn advent2() {
    fn part1(input: String) -> u32 {
        return input.split('\n').map(|line| {
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
            return max - min;
        }).sum();
    }

    fn part2(input: String) -> u32 {
        return input.split('\n').map(|line| {
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
            return res;
        }).sum();
    }

    let sample = "5 1 9 5
7 5 3
2 4 6 8".to_string();
    assert_eq!(part1(sample), 18);

    let sample2 = "5 9 2 8
9 4 7 3
3 8 6 5".to_string();
    assert_eq!(part2(sample2), 9);

    let input = read_file("data/day2.txt");
    println!("day 2 - 1 {}", part1(input.to_string()));
    println!("day 2 - 2 {}", part2(input.to_string()));
}

use std::collections::HashMap;
fn advent3() {
    fn dist_vec(input: u32) -> (i32, i32) {
        /*
        shells: 1^2, 3^2, 5^2, 7^2, ...
        we can find bottom right corners, and we know length of legs
        total manhatten distance is number of shells + perpendicular offset
        */
        if input == 1 {
            return (0, 0);
        }
        let shells = (1..)
            .filter(|n| n % 2 == 1)
            .map(|n| n*n)
            .take_while(|&n| n < input)
            .collect::<Vec<u32>>();
        let last_corner = shells.last().cloned().unwrap();
        let num_shells = shells.len() as i32;
        let leg_length = (last_corner as f64).sqrt().abs() as u32 + 2;
        let leg_middle = (((leg_length - 1) / 2) as i32).abs();

        let rest = input - last_corner;
        let which_leg = (rest / (leg_length - 1)) as i32;
        let pos_on_leg = (rest % (leg_length - 1)) as i32; 
        let leg_offset = pos_on_leg - leg_middle;
        match which_leg {
            0 => (num_shells, leg_offset),
            1 => (-leg_offset, num_shells),
            2 => (-num_shells, -leg_offset),
            3 => (leg_offset, -num_shells),
            4 => (num_shells, leg_offset),
            _ => panic!("invalid leg")
        }
    }

    fn dist(input: u32) -> u32 {
        let (a, b) = dist_vec(input);
        return (a.abs() + b.abs()) as u32;
    }

    fn is_neighbor(v1: (i32, i32), v2: (i32, i32)) -> bool {
        let diff = ((v1.0 - v2.0).abs(), 
                      (v1.1 - v2.1).abs());
        match diff {
            (0, 1) => true,
            (1, 0) => true,
            (1, 1) => true,
            _ => false
        }
    }

    /// better way would be to programmatically find "cell" at "coord"
    fn brute_neighbors(input: u32) -> Vec<u32> {
        let pos = dist_vec(input);
        (1..input)
            .filter(|&n| is_neighbor(pos, dist_vec(n)))
            .collect::<Vec<_>>()
    }

    fn part2(target: u32) -> u32 {
        let mut cell_values : HashMap<u32, u32> = HashMap::new();
        cell_values.insert(1, 1);
        for i in 2.. {
            let neighbors = brute_neighbors(i);
            let val = neighbors
                .iter()
                .map(|n| cell_values.get(n).unwrap())
                .sum();
            cell_values.insert(i, val);
            if val > target {
                return val;
            }
        }
        return 0;
    }

    for &(target, steps) in [
        (1, 0),
        (12, 3),
        (23, 2),
        (21, 4),
        (1024, 31)
    ].iter() {
        assert_eq!(dist(target), steps);
    }

    assert_eq!(part2(747), 806);

    let input : u32 = 361527;
    println!("day 3 - 1 {}", dist(input));
    println!("day 3 - 2 {}", part2(input));
}

use std::collections::HashSet;
use std::iter::FromIterator;

fn advent4() {
    fn sort_string(s : String) -> String {
        return s
            .chars()
            .sorted()
            .iter()
            .collect::<String>();
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
        return words_len == unique_words.len();
    }

    fn worker(passwords: String, is_part1: bool) -> usize {
        return passwords.split('\n').map(|line| {
            if is_valid(line.to_string(), is_part1) {1} else {0}
        }).sum();
    }

    for &(sample, valid) in [
        ("aa bb cc dd ee", true),
        ("aa bb cc dd aa", false),
        ("aa bb cc dd aaa", true)
    ].iter() {
        assert_eq!(is_valid(sample.to_string(), true), valid);
    }

    for &(sample, valid) in [
        ("abcde fghij", true),
        ("abcde xyz ecdab", false),
        ("a ab abc abd abf abj", true),
        ("iiii oiii ooii oooi oooo", true),
        ("oiii ioii iioi iiio", false)
    ].iter() {
        assert_eq!(is_valid(sample.to_string(), false), valid);
    }

    let input = read_file("data/day4.txt");
    println!("day 4 - 1 {}", worker(input.clone(), true));
    println!("day 4 - 2 {}", worker(input, false));
}

fn main() {
    advent1();
    advent2();
	advent3();
    advent4();
}
