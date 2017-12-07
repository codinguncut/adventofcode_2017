use common;
use regex::Regex;
use std::collections::HashSet;


#[derive(Debug, PartialEq)]
struct Program {
    name: String,
    weight: u32,
    children: Vec<String>,
}


/// parse single line using regular expressions
fn parse_line(input: &str) -> Program {
    lazy_static! {
        static ref RE : Regex = Regex::new(
            r"^(\w+) \((\d+)\)").unwrap();
        static ref RE_CHILDREN : Regex = Regex::new(
            r"(\w+) \((\d+)\) -> ([\w, ]+)").unwrap();
    }

    // FIXME: optional named group for children
    if input.contains(" -> ") {
        let cap = RE_CHILDREN.captures(input).unwrap();
        let (name, weight, children) = (&cap[1], &cap[2], &cap[3]);
        Program{name: name.to_string(),
                weight: weight.parse().unwrap(),
                children: children.split(", ").map(|n| n.to_string()).collect(),
        }
    } else {
        let cap = RE.captures(input).unwrap();
        let (name, weight) = (&cap[1], &cap[2]);
        Program{name: name.to_string(),
                weight: weight.parse().unwrap(),
                children: vec![]
        }
    }
}


/// parse input lines into Vector of Programs
fn parse_lines(input: &str) -> Vec<Program> {
    input.lines()
        .map(parse_line)
        .collect()
}


/// find root name of the tree, i.e. single node with no parent
fn find_root(input: &str) -> String {
    let programs = parse_lines(&input);
    let mut have_parent : HashSet<String> = HashSet::new();
    let all_programs : HashSet<String> = programs
        .iter()
        .map(|n| n.name.clone())
        .collect();
    for program in &programs {
        for child in &program.children {
            have_parent.insert(child.clone());
        }
    }
    let diff : Vec<String> = all_programs
        .difference(&have_parent)
        .cloned()
        .collect();
    assert!(diff.len() == 1);
    diff[0].clone()
}


#[allow(dead_code)]
pub fn main() {
    let input = common::read_file("data/day7.txt");
    println!("day 7 - 1: {}", find_root(&input));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("pbga (66)"),
                   Program{name: "pbga".to_string(),
                           weight: 66,
                           children: vec![]});
        assert_eq!(parse_line("fwft (72) -> ktlj, cntj, xhth"),
                   Program{name: "fwft".to_string(),
                           weight: 72,
                           children: vec!["ktlj".to_string(),
                                          "cntj".to_string(),
                                          "xhth".to_string()]});
    }

    #[test]
    fn test_find_root() {
        let sample_input = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";

        assert_eq!(find_root(&sample_input),
                   "tknk".to_string());
    }

    #[test]
    fn test_solution1() {
        let input = common::read_file("data/day7.txt");
        assert_eq!(find_root(&input), "uownj");
    }
}
