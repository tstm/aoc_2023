#![allow(dead_code, unused_variables)]

// use std::cell::RefCell;
use std::collections::BTreeMap;

// use rayon::prelude::*;

struct Next {
    left: String,
    right: String,
}

impl Next {
    const PARENTHESIS: &[char] = &['(', ')'];
    // Takes in "(foo, bar)"
    fn new(input: &str) -> Next {
        let (left, right) = input
            .trim_matches(Next::PARENTHESIS)
            .split_once(", ")
            .unwrap();
        Next {
            left: left.into(),
            right: right.into(),
        }
    }
}

pub fn run(input: &str) -> Result<usize, String> {
    let mut it = input.lines();
    // let mut nodes = BTreeMap::new();

    let instructions = it.next().unwrap();
    let nodes = it
        .skip(1)
        .map(|line| {
            let (node_id, rest) = line.split_once(" = ").unwrap();
            let next = Next::new(rest);
            // nodes.insert(node_id, next);
            (node_id.to_string(), next)
        })
        .collect::<BTreeMap<String, Next>>();

    let mut it = instructions.chars().cycle();

    let mut steps = 0;
    let mut node = nodes.get("AAA").expect("Start node should always be there");
    loop {
        steps += 1;
        let next_node;
        if it.next().unwrap() == 'L' {
            next_node = &node.left;
        } else {
            next_node = &node.right;
        }
        if !(next_node == "ZZZ") {
            node = nodes
                .get(next_node)
                .expect("There should always be a node to find");
        } else {
            break;
        }
    }

    Ok(steps)
}
