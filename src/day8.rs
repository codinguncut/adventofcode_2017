use common;
use itertools::Itertools;
use std::collections::HashMap;

type State = HashMap<String, i32>;
type StateVec = Vec<(String, i32)>;

fn find_largest(vec: &StateVec) -> i32 {
    *vec.iter().map(|x| x.1).sorted().last().unwrap()
}

fn state_to_vec(state: &State) -> StateVec {
    state.iter().map(|(s, n)| (s.to_string(), *n)).sorted()
}

fn single_step(input: &str, mut state: &mut State) -> State {
    let parts : Vec<&str> = input.split_whitespace().collect();
    let (reg1, op, ival1, cif, reg2, cmp, ival2) = (
        parts[0], parts[1], parts[2].parse().unwrap(), parts[3], parts[4], parts[5], 
        parts[6].parse().unwrap());
    assert_eq!(cif, "if");

    fn fetch(reg: &str, state: &State) -> i32 {
      *state.get(reg).unwrap_or(&0)
    }

    fn inc(reg: &str, val: i32, state: &mut State) {
      *state.entry(reg.to_string()).or_insert(0) += val;
    }

    let rv2 = fetch(reg2, &state);
    // TODO: PartialEq::gt
    let cond = match cmp {
      ">" => rv2 > ival2,
      "<" => rv2 < ival2,
      ">=" => rv2 >= ival2,
      "<=" => rv2 <= ival2,
      "==" => rv2 == ival2,
      "!=" => rv2 != ival2,
      _ => panic!("unknown comparison operator"),
    };

    if cond {
      let to_add : i32 = match op {
        "inc" => ival1,
        "dec" => -ival1,
        _ => panic!("unknown inc/dec operator"),
      };
      inc(reg1, to_add, &mut state);
    }
    state.clone()
}

fn run_program(input: &str) -> StateVec {
    let mut state: State = HashMap::new();
    for line in input.lines() {
        single_step(&line, &mut state);
    }
    state.iter().map(|(s, n)| (s.to_string(), *n)).sorted()
}

fn find_any_largest(input: &str) -> i32 {
    let mut state: State = HashMap::new();
    *input.lines().map(|line| {
        single_step(&line, &mut state);
        let vec = state_to_vec(&state);
        find_largest(&vec)
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
