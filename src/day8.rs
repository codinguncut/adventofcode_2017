use common;
use itertools::Itertools;
use std::collections::HashMap;

type State = HashMap<String, i32>;
type StateVec = Vec<(String, i32)>;


/// find largest register value in StateVec
fn find_largest(vec: &StateVec) -> i32 {
    *vec.iter()
        .map(|x| x.1)
        .sorted()
        .last()
        .unwrap()
}


/// convert HashMap to StateVec for easier comparison/testing
fn state_to_vec(state: &State) -> StateVec {
    state.iter()
        // FIXME: better way?
        .map(|(s, n)| (s.to_string(), *n))
        .sorted()
}


/// FIXME: no State cloning
/// perform single computation step
fn single_step(input: &str, mut state: &mut State) -> State {
    let parts: Vec<&str> = input.split_whitespace().collect();
    // FIXME: better way to destructure? collect tuple/array?
    let (reg1, op, ival1, cif, reg2, cmp, ival2) = (
        parts[0], parts[1], parts[2].parse().unwrap(), parts[3], parts[4], parts[5], 
        parts[6].parse().unwrap());
    assert_eq!(cif, "if");

    // FIXME: mut closure over state
    /// fetch value of register "reg", or 0
    fn fetch(reg: &str, state: &State) -> i32 {
      *state.get(reg).unwrap_or(&0)
    }

    /// increment register "reg" by "val"
    fn inc(reg: &str, val: i32, state: &mut State) {
      *state.entry(reg.to_string()).or_insert(0) += val;
    }

    let cmp_op = match cmp {
      ">" => i32::gt,
      "<" => i32::lt,
      ">=" => i32::ge,
      "<=" => i32::le,
      "==" => i32::eq,
      "!=" => i32::ne,
      _ => panic!("unknown comparison operator"),
    };

    let rv2 = fetch(reg2, &state);
    if cmp_op(&rv2, &ival2) {
      let to_add: i32 = match op {
        "inc" => ival1,
        "dec" => -ival1,
        _ => panic!("unknown inc/dec operator"),
      };
      inc(reg1, to_add, &mut state);
    }
    state.clone()
}


/// run whole program of one or multiple instructions
fn run_program(input: &str) -> StateVec {
    let mut state: State = HashMap::new();
    for line in input.lines() {
        single_step(&line, &mut state);
    }
    state.iter().map(|(s, n)| (s.to_string(), *n)).sorted()
}


/// part2 - find intermediate largest register value
fn find_any_largest(input: &str) -> i32 {
    let mut state: State = HashMap::new();
    *input.lines().map(|line| {
        single_step(&line, &mut state);
        find_largest(&state_to_vec(&state))
    }).sorted().last().unwrap()
}

pub fn main() {
    let input = common::read_file("data/day8.txt");
    println!("day 8 - 1 {}", find_largest(&run_program(&input)));
    println!("day 8 - 2 {}", find_any_largest(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single() {
        assert_eq!(run_program("b inc 5 if a < 5"),
                   vec![("b".to_string(), 5)]);
        assert_eq!(run_program("b inc 5 if a > 1"),
                   vec![]);
    }

    #[test]
    fn test_sample() {
        let sample_input = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";
        assert_eq!(run_program(&sample_input),
                   vec![("a".to_string(), 1),
                        ("c".to_string(), -10)]);
    }

    #[test]
    fn test_solutions() {
        let input = common::read_file("data/day8.txt");
        assert_eq!(find_largest(&run_program(&input)), 5849);
        assert_eq!(find_any_largest(&input), 6702);
    }
}
