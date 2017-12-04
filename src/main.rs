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


#[allow(dead_code)]
fn advent1() {
    // get characters that are the same in 's' with distance 'offset'
    fn get_same(s: String, offset: usize) -> Vec<char> {
        let t = [&s[offset..], &s[..offset]].join("");
        let x = s.chars().zip(t.chars())
            .filter(|&(a, b)| a == b)
            .map(|(a, _)| a)
            .collect::<Vec<_>>();
        return x;
    }

    // turn vector of characters into vector of numbers
    // Note: no error handling
    fn to_num(a: Vec<char>) -> Vec<u32> {
        return a
            .iter()
            .map(|c| c.to_digit(10)
            .unwrap())
            .collect::<Vec<_>>();
    }

    let input = read_file("data/day1.txt");
    let offset = input.chars().count() / 2 as usize;
    let res : u32 = to_num(get_same(input.to_string(), offset))
        .iter()
        .sum();
    println!("{}", res);
}


#[allow(dead_code)]
fn advent2() {
    let input = read_file("data/day2.txt");

    fn part1(input: String) {
        let sum : u32 = input.split('\n').map(|line| {
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
        println!("{}", sum);
    }
    part1(input.to_string());

    /*
    let sum : u32 = input.split('\n').map(|line| {
        let div : u32 = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
			.tuple_combinations()
			.filter(|(a, b)| a % b == 0)
			.map(|(a, b)| a / b as u32)
			.next()
			.unwrap();
        return div;
    }).sum();
    println!("{}", sum);
    */
}

#[allow(dead_code)]
fn squares(input: u32) {
	/*
	shells: 1^2, 3^2, 5^2, 7^2, ...
	we can find bottom right corners, and we know length of legs
	*/
	let shells = (1..)
        .filter(|n| n % 2 == 1)
        .map(|n| n*n)
        .take_while(|&n| n < input)
        .collect::<Vec<u32>>();
	println!("{:?}", shells);
}

#[allow(dead_code)]
fn advent3() {
	//let input : u32 = 361527;
}

use std::collections::HashSet;
use std::iter::FromIterator;

fn reverse_string(s : String) -> String {
    return s
        .chars()
        .sorted()
        .iter()
        .collect::<String>();
}

#[allow(dead_code)]
fn advent4() {
    fn is_valid(password: String, is_part1 : bool) -> bool {
        let words = password 
            .trim()
            .split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let words_len = words.clone().len();
        if is_part1 {
            let unique_words : HashSet<String> = HashSet::from_iter(
                words.iter().cloned()
            );
            return words_len == unique_words.len();
        } else {
            let unique_words : HashSet<String> = HashSet::from_iter(
                words
                .iter()
                .cloned()
                .map(reverse_string)
            );
            return words_len == unique_words.len();
        }
    }

    fn worker(passwords: String, is_part1: bool) -> usize {
        let count : usize = passwords.split('\n').map(|line| {
            if is_valid(line.to_string(), is_part1) {1} else {0}
        }).sum();
        return count;
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
    println!("{}", worker(input.clone(), true));
    println!("{}", worker(input, false));
}

fn main() {
    //advent1();
    //advent2();
	//squares(1024);
    advent4();
}
