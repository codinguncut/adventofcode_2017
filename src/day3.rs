use std::collections::HashMap;

/// calculate vector from origin to cell "input"
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
    let leg_length = (last_corner as f64).sqrt() as u32 + 2;
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


/// calculate manhatten distance from origin to cell "input"
fn dist(input: u32) -> u32 {
    let (a, b) = dist_vec(input);
    (a.abs() + b.abs()) as u32
}


// test if "v1" and "v2" are manhatten neighbors
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


/// brute force find all neighbors of cell "input" by iterating over all
///     predecessors.
/// FIXME: better way would be to programmatically find "cell" at "coord"
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
        for &(target, steps) in [

            (1, 0),
            (12, 3),
            (23, 2),
            (21, 4),
            (1024, 31)
        ].iter() {
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
