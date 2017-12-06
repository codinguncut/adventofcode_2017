use std::collections::HashMap;

struct Point(i32, i32);

impl Point {
    // NOTE: std::ops::Sub had signature I didn't like
    fn minus(&self, other : &Point) -> Point {
        Point(self.0 - other.0,
              self.1 - other.1)
    }

    fn abs(&self) -> Point {
        Point(self.0.abs(),
              self.1.abs())
    }
}


// FIXME: u32::is_even
fn odd(n: &u32) -> bool {
    n % 2 == 1
}


/// calculate vector from origin to cell "input"
fn dist_vec(input: u32) -> Point {
    /*
    shells: 1^2, 3^2, 5^2, 7^2, ...
    we can find bottom right corners, and we know length of legs
    total manhatten distance is number of shells + perpendicular offset
    */
    if input == 1 {
        return Point(0, 0);
    }
    let shells : Vec<u32> = (1..)
        // FIXME: step_by(2)?
        .filter(odd)
        .map(|n| n*n)
        .take_while(|&n| n < input)
        .collect();
    let num_shells = shells.len() as i32;
    let last_corner = shells[num_shells as usize - 1];
    let leg_length = (last_corner as f64).sqrt() as u32 + 2;
    let leg_middle = (((leg_length - 1) / 2) as i32).abs();

    let rest = input - last_corner;
    let which_leg = (rest / (leg_length - 1)) as i32;
    let pos_on_leg = (rest % (leg_length - 1)) as i32; 
    let leg_offset = pos_on_leg - leg_middle;
    match which_leg {
        0 => Point(num_shells, leg_offset),
        1 => Point(-leg_offset, num_shells),
        2 => Point(-num_shells, -leg_offset),
        3 => Point(leg_offset, -num_shells),
        4 => Point(num_shells, leg_offset), // bottom right corner
        _ => panic!("invalid leg")
    }
}


/// calculate manhatten distance from origin to cell "input"
fn dist(input: u32) -> u32 {
    let Point(a, b) = dist_vec(input);
    (a.abs() + b.abs()) as u32
}


/// test if "v1" and "v2" are manhatten neighbors
fn is_neighbor(v1: &Point, v2: &Point) -> bool {
    let diff = v1.minus(v2).abs();
    match diff {
        Point(0, 1) => true,
        Point(1, 0) => true,
        Point(1, 1) => true,
        _ => false
    }
}


/// brute force find all neighbors of cell "input" by iterating over all
///     predecessors.
/// FIXME: better way would be to programmatically find "cell" at "coord"
fn brute_neighbors(input: u32) -> Vec<u32> {
    let pos = dist_vec(input);
    (1..input)
        .filter(|&n| is_neighbor(&pos, &dist_vec(n)))
        .collect()
}


fn part2(target: u32) -> u32 {
    let mut cell_values : HashMap<u32, u32> = HashMap::new();
    cell_values.insert(1, 1);
    for i in 2.. {
        let val = brute_neighbors(i)
            .iter()
            .map(|n| cell_values.get(n).unwrap())
            .sum();
        cell_values.insert(i, val);
        if val > target {
            return val;
        }
    }
    panic!("total function");
}


#[allow(dead_code)]
pub fn main() {
    let input : u32 = 361527;
    println!("day 3 - 1 {}", dist(input));
    println!("day 3 - 2 {}", part2(input));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        for &(target, steps) in &[
            (1, 0),
            (12, 3),
            (23, 2),
            (21, 4),
            (1024, 31)
        ] {
            assert_eq!(dist(target), steps);
        }
    }

    #[test]
    fn test2() {
        assert_eq!(part2(747), 806);
    }

    #[test]
    fn solution1() {
        let input : u32 = 361527;
        assert_eq!(dist(input), 326);
    }

    #[test]
    fn solution2() {
        let input : u32 = 361527;
        assert_eq!(part2(input), 363010);
    }
}
