#![allow(dead_code, unused_variables)]

use std::cell::RefCell;
use std::collections::BTreeSet;

enum Direction {
    Left,
    Right,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
struct Node {
    id: String,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(id: &str) -> Node {
        Node {
            // id: id.chars().collect::<Vec<char>>().try_into().unwrap(),
            id: id.to_string(),
            left: None,
            right: None,
        }
    }

    fn set_both(&mut self, left: Node, right: Node) {
        self.left = Some(Box::new(left));
        self.right = Some(Box::new(right));
    }
}

// use rayon::prelude::*;

pub fn run(input: &str) -> Result<usize, String> {
    let parenthesis: &[_] = &['(', ')'];
    let mut nodes = Vec::new();
    let associations: Vec<(_, _, _)> = input
        .lines()
        .skip(2)
        .map(|line| {
            let (node_id, rest) = line.split_once(" = ").unwrap();
            let (left, right) = rest.trim_matches(parenthesis).split_once(", ").unwrap();
            nodes.push(Node::new(node_id));
            (node_id, left, right)
        })
        .collect();

    for (node_id, left, right) in associations.into_iter() {
        eprintln!("Node: {} Left: {} Right {}", &node_id, &left, &right);
        let left_id = nodes.iter().position(|n| n.id == left).unwrap();
        let right_id = nodes.iter().position(|n| n.id == right).unwrap();
        let left = nodes.swap_remove(left_id);
        let right = nodes.swap_remove(right_id);
        let id = nodes.iter().position(|n| n.id == node_id).unwrap();
        nodes.get_mut(id).unwrap().set_both(left, right);
    }

    dbg!(nodes);

    Ok(0)
}
