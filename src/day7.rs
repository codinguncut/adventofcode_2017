use common;
use regex::Regex;
use std::collections::HashSet;
use std::collections::HashMap;


#[derive(Debug, Clone, PartialEq)]
struct Program {
    name: String,
    weight: u32,
    children: Vec<String>,
}


/// parse single line using regular expressions
fn parse_line(input: &str) -> Program {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(?P<name>\w+) \((?P<weight>\d+)\)(?: -> (?P<children>[\w, ]+))?").unwrap();
    }

    let cap = RE.captures(input).unwrap();
    let (name, weight) = (cap.name("name").unwrap().as_str(),
                          cap.name("weight").unwrap().as_str());
    let children = match cap.name("children") {
        Some(x) => x.as_str()
            .split(", ")
            .map(|n| n.to_string())
            .collect(),
        _ => vec![]
    };
    Program{name: name.to_string(),
            weight: weight.parse().unwrap(),
            children}
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


fn check_weights(input: &str) -> (u32, u32) {
    let programs = parse_lines(&input);
    let mut weights : HashMap<&str, &Program> = HashMap::new();
    for program in &programs {
        weights.insert(&program.name, &program);
    }


    /// depth first iterator
    fn dfs(node: &str, weights: &HashMap<&str, &Program>) -> Vec<Program> {
        let program = weights
            .get(node)
            .unwrap();
        let res = vec![(**program).clone()];
        if program.children.len() > 0 {
            let cs : Vec<Program> = program.children.iter().map(|child| {
                dfs(child, &weights)
            }).flat_map(|n| n).collect();
            cs.iter().chain(res.iter()).cloned().collect()
        } else {
            res
        }
    }


    // calculate child weights recursively
    // FIXME: extremely unhappy with the println approach of this
    // FIXME: need depth-first iterator instead
    fn rec(node: &str, weights: &HashMap<&str, &Program>) -> (u32, u32) {
        let program = weights
            .get(node)
            .unwrap();
        if program.children.len() > 0 {
            let child_weights : Vec<(u32, u32)> = program.children.iter().map(|child| {
                rec(&child, &weights)
            }).collect();
            if !child_weights.iter().all(|&(_, n)| &child_weights[0].1 == &n) {
                println!("imbalance {:?}", child_weights);
            }
            let accum : u32 = child_weights.iter().map(|a| a.1).sum();
            (program.weight, program.weight + accum)
        } else {
            (program.weight, program.weight)
        }
    }

    let root = find_root(&input);
    rec(&root, &weights)
}


#[allow(dead_code)]
pub fn main() {
    let input = common::read_file("data/day7.txt");
    println!("day 7 - 1: {}", find_root(&input));
    check_weights(&input);
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
