use itertools::Itertools;
use std::collections::HashSet;

type Banks = Vec<usize>;


/// find index of largest element in `banks`
fn find_largest(banks: &Banks) -> usize {
    // FIXME: .position?
    banks.iter().enumerate().fold(None, |state, (i, n)| match state {
        None => Some((i, n)),
        Some((i_max, n_max)) =>
            if n > n_max {
                Some((i, n))
            } else {
                Some((i_max, n_max))
            }
    }).unwrap().0
}


/// reallocate blocks in bank `which` to other banks
fn reallocate(banks: &mut Banks, which: usize) -> &Banks {
    let blocks = *banks.get(which).unwrap();
    banks[which] = 0;
    (0..blocks).foreach(|n| {
            let idx = (which + n + 1) % banks.len();
            banks[idx] += 1;
    });
    banks // for testing purposes
}


/// iterate reallocations; returns vector of bank representations until
///     loop occurs.
fn iterate(banks: &mut Banks) -> Vec<String> {
    let mut seen : HashSet<String> = HashSet::new();
    seen.insert(format!("{:?}", banks));

    (1..).map(|_| {
        let largest = find_largest(&banks);
        reallocate(banks, largest);
        format!("{:?}", banks)
    }).scan(false, |state, x| {
        let new_state = seen.contains(&x);
        let res = if *state {
            None
        } else {
            seen.insert(x.clone());
            Some(x)
        };
        *state = new_state;
        res
    }).collect()
}
    

/// find longest occurring cycle within intermediate reallocation
fn part2(banks: &mut Banks) -> usize {
    let vec = iterate(banks);
    let pos_last = vec.len() - 1;
    let last = &vec[pos_last];
    let first = vec.iter().position(|n| n == last).unwrap();
    pos_last - first
}


#[allow(dead_code)]
pub fn main() {
    let mut input = vec![
        14, 0, 15, 12, 11, 11, 3, 5, 1, 6, 8, 4, 9, 1, 8, 4];
    let mut copy = input.clone();
    println!("day 6 - 1: {}", iterate(&mut input).len());
    println!("day 6 - 2: {}", part2(&mut copy));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_largest() {
        assert_eq!(find_largest(&vec![0, 2, 7, 0]), 2);
        assert_eq!(find_largest(&vec![2, 4, 1, 2]), 1);
    }

    #[test]
    fn test_reallocate() {
        let res : Vec<usize> = vec![2, 4, 1, 2];
        assert_eq!(reallocate(&mut vec![0, 2, 7, 0], 2), &res);
    }

    #[test]
    fn test_iterate() {
        let res = iterate(&mut vec![0, 2, 7, 0]);
        assert_eq!(res, vec![
             format!("{:?}", vec![2, 4, 1, 2]),
             format!("{:?}", vec![3, 1, 2, 3]),
             format!("{:?}", vec![0, 2, 3, 4]),
             format!("{:?}", vec![1, 3, 4, 1]),
             format!("{:?}", vec![2, 4, 1, 2]),
        ]);
    }

    #[test]
    fn test_part2() {
        let mut res = vec![0, 2, 7, 0];
        assert_eq!(part2(&mut res), 4);
    }

    #[test]
    fn test_solution1() { 
        let mut input = vec![
            14, 0, 15, 12, 11, 11, 3, 5, 1, 6, 8, 4, 9, 1, 8, 4];
        assert_eq!(iterate(&mut input).len(), 11137);
    }

    #[test]
    fn test_solution2() {
        let mut input = vec![
            14, 0, 15, 12, 11, 11, 3, 5, 1, 6, 8, 4, 9, 1, 8, 4];
        assert_eq!(part2(&mut input), 1037);
    }
}
