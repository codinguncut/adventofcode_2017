use common;

/// struct maintaining "machine state"
/// `instr` - vector of signed jump offsets
/// `index` - current index into the `instr` table
/// `count` - executed number of jumps so far
#[derive(Debug, Clone)]
struct State {
    instr: Vec<i32>,
    index: usize,
    count: usize 
}

/// execute single step against mutable "state" with "update" function
///
/// # Arguments
/// `state` - mutable state struct
/// `update`- function to update jump offset
///
/// # Returns
/// `None` if step was successful
/// `Some(usize)` if accessing element outside instructions
fn step(state: &mut State, update: &Fn(i32) -> i32) -> Option<usize> {
    let instr = &mut state.instr;
    let jmp : i32 = *match (&instr).get(state.index) {
      Some(x) => x,
      _ => return Some(state.count)
    };
    instr[state.index] = update(jmp);
    state.index = (state.index as i32 + jmp) as usize;
    state.count += 1;
    None
}


/// run single steps until end condition is reached
fn run(state: &mut State, is_part1: bool) -> usize {
    let update : fn(i32) -> i32 = if is_part1 {
        |n| n + 1
    } else {
        |n| if n >= 3 {n-1} else {n+1}
    };
    loop {
        match step(state, &update) {
            Some(x) => return x,
            _ => continue
        }
    }
}


/// read initial state from file and return `State`
fn read_state() -> State {
    let input = common::read_file("data/day5.txt");
    let instr : Vec<i32> = input
        .trim()
        .lines()
        .map(|n| n.parse().unwrap())
        .collect();
    State{instr, index: 0, count: 0}
}


pub fn main() {
    let mut state = read_state();

    // keep original to reset for second run
    let state_copy = state.clone();

    println!("day 5 - 1: {}", run(&mut state, true));

    let mut state = state_copy;
    println!("day 5 - 2: {}", run(&mut state, false));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step() {
        let mut state = State{instr: vec![0, 3, 0, 1, -3],
                              index: 0,
                              count: 0};
        let update = |n| n + 1;

        let results = (1..).map(|_| {
            match step(&mut state, &update) {
                None => Some(state.instr.clone()),
                _ => None
            }
        }).take_while(Option::is_some).map(|n| n.unwrap());
        let sequence = [
            vec![1, 3, 0, 1, -3],
            vec![2, 3, 0, 1, -3],
            vec![2, 4, 0, 1, -3],
            vec![2, 4, 0, 1, -2],
            vec![2, 5, 0, 1, -2],
        ];
        assert!(results.zip(sequence.iter()).all(|(a, b)| a == *b));
    }

    #[test]
    fn test_run() {
        let mut state = State{instr: vec![0, 3, 0, 1, -3],
                              index: 0,
                              count: 0};
        assert_eq!(run(&mut state, true), 5);
    }

    #[test]
    fn test_run2() {
        let mut state = State{instr: vec![0, 3, 0, 1, -3],
                              index: 0,
                              count: 0};
        assert_eq!(run(&mut state, false), 10);
    }

    #[test]
    fn solution1() {
        let mut state = read_state();
        assert_eq!(run(&mut state, true), 387096);
    }

    //#[test]
    #[allow(dead_code)]
    fn solution2() {
        let mut state = read_state();
        assert_eq!(run(&mut state, false), 28040648);
    }
}
