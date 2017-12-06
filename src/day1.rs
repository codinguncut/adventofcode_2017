use common;

// get matching characters in "s" that are "offset" apart
fn get_matching(s: &str, offset: usize) -> Vec<char> {
    let t = [&s[offset..], &s[..offset]].join("");
    s.chars()
        .zip(t.chars())
        .filter(|&(a, b)| a == b)
        .map(|(a, _)| a)
        .collect()
}


// turn vector of characters into vector of (digit) numbers
fn to_num(a: &Vec<char>) -> Vec<u32> {
    a.iter()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}


fn worker(s: &str, offset: usize) -> u32 {
    to_num(&get_matching(s, offset))
        .iter()
        .sum()
}


#[allow(dead_code)]
pub fn main() {
    let input = common::read_file("data/day1.txt");
    let offset = &input.chars().count() / 2 as usize;
    println!("day 1 - 1 {}", worker(&input, 1));
    println!("day 1 - 2 {}", worker(&input, offset));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        for &(sample, valid) in &[
            ("1122", 3),
            ("1111", 4),
            ("1234", 0),
            ("91212129", 9)
        ] {
            assert_eq!(worker(sample, 1), valid);
        }
    }

    #[test]
    fn test2() {
        for &(sample, valid) in &[
            ("1212", 6),
            ("1221", 0),
            ("123425", 4),
            ("123123", 12),
            ("12131415", 4),
        ] {
            let offset = sample.chars().count() / 2 as usize;
            assert_eq!(worker(sample, offset), valid);
        }
    }

    #[test]
    fn solution1() {
        let input = common::read_file("data/day1.txt");
        assert_eq!(worker(&input, 1), 995);
    }

    #[test]
    fn solution2() {
        let input = common::read_file("data/day1.txt");
        let offset = &input.chars().count() / 2 as usize;
        assert_eq!(worker(&input, offset), 1130);
    }
}
