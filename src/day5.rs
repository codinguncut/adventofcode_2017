use common;

#[derive(Debug)]
struct State {
    instr: Vec<i32>,
    index: usize,
    count: usize 
}

// TODO: implement single step
// TODO: state struct
fn step(state: &mut State, update: &Fn(i32) -> i32) -> Option<usize> {
    let instr = &mut state.instr;
    let jmp : i32 = *match (&instr).get(state.index) {
      Some(x) => x,
      None => return Some(state.count)
    };
    instr[state.index] = update(jmp);
    state.index = (state.index as i32 + jmp) as usize;
    state.count += 1;
    None
}


fn run(state: &mut State, is_part1: bool) -> usize {
    let update : fn(i32) -> i32 = if is_part1 {
        |n| n + 1
    } else {
        |n| if n >= 3 {n-1} else {n+1}
    };
    loop {
        match step(state, &update) {
            None => continue,
            Some(x) => return x
        }
    }
}


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
    let instr_copy = state.instr.clone();

    println!("day 5 - 1: {}", run(&mut state, true));

    state.instr = instr_copy;
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
        println!("{:?}", &state);

        step(&mut state, &update);
        println!("{:?}", &state);
        assert_eq!(vec![1, 3, 0, 1, -3], state.instr);

        step(&mut state, &update);
        println!("{:?}", &state);
        assert_eq!(vec![2, 3, 0, 1, -3], state.instr);

        step(&mut state, &update);
        println!("{:?}", &state);
        assert_eq!(vec![2, 4, 0, 1, -3], state.instr);

        step(&mut state, &update);
        println!("{:?}", &state);
        assert_eq!(vec![2, 4, 0, 1, -2], state.instr);

        step(&mut state, &update);
        println!("{:?}", &state);
        assert_eq!(vec![2, 5, 0, 1, -2], state.instr);
    }

    #[test]
    fn test_run() {
        let mut state = State{instr: vec![0, 3, 0, 1, -3], index: 0, count: 0};
        assert_eq!(run(&mut state, true), 5);
    }

    #[test]
    fn test_run2() {
        let mut state = State{instr: vec![0, 3, 0, 1, -3], index: 0, count: 0};
        assert_eq!(run(&mut state, false), 10);
    }

    #[test]
    fn solution1() {
        let mut state = read_state();
        assert_eq!(run(&mut state, true), 387096);
    }

    //#[test]
    fn solution2() {
        let mut state = read_state();
        assert_eq!(run(&mut state, false), 28040648);
    }
}
