#![allow(dead_code, unused_variables)]

// use rayon::prelude::*;

use std::collections::HashMap;

#[derive(PartialEq, Eq, Debug)]
enum Operation {
    GreaterThan,
    LessThan,
}

impl From<char> for Operation {
    fn from(input: char) -> Operation {
        match input {
            '<' => Operation::LessThan,
            '>' => Operation::GreaterThan,
            _ => panic!("Only two operations supported."),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Next {
    Accept,
    Reject,
    Forward(String),
}

impl From<&str> for Next {
    fn from(input: &str) -> Next {
        match input {
            "R" => Next::Reject,
            "A" => Next::Accept,
            fwd => Next::Forward(fwd.to_string()),
        }
    }
}

#[derive(Debug)]
struct Rule {
    symbol: char,
    limit: usize,
    op: Operation,
    result: Next,
}

impl Rule {
    fn evaluate(&self, part: &Part) -> Option<&Next> {
        let value = match self.symbol {
            'x' => part.x,
            'm' => part.m,
            'a' => part.a,
            's' => part.s,
            _ => panic!("No such value for evaluation"),
        };
        match self.op {
            Operation::GreaterThan => {
                if value > self.limit {
                    Some(&self.result)
                } else {
                    None
                }
            }
            Operation::LessThan => {
                if value < self.limit {
                    Some(&self.result)
                } else {
                    None
                }
            }
        }
    }
}

impl From<&str> for Rule {
    fn from(input: &str) -> Rule {
        let symbol = input.chars().nth(0).unwrap();
        let op = Operation::from(input.chars().nth(1).unwrap());
        let (limit, result) = input[2..].split_once(":").unwrap();
        let limit = limit.parse::<usize>().unwrap();
        let result = Next::from(result);

        Rule {
            symbol,
            limit,
            op,
            result,
        }
    }
}

#[derive(Debug)]
struct Workflow {
    rules: Vec<Rule>,
    default_result: Next,
}

impl Part {
    fn run_workflow(
        &self,
        current_workflow: &Workflow,
        workflows: &HashMap<String, Workflow>,
    ) -> bool {
        for rule in current_workflow.rules.iter() {
            match rule.evaluate(self) {
                Some(next) => match next {
                    Next::Accept => return true,
                    Next::Reject => return false,
                    Next::Forward(fwd) => {
                        return self.run_workflow(workflows.get(fwd).unwrap(), workflows)
                    }
                },
                None => {}
            };
        }
        match &current_workflow.default_result {
            Next::Accept => return true,
            Next::Reject => return false,
            Next::Forward(fwd) => return self.run_workflow(workflows.get(fwd).unwrap(), workflows),
        }
    }

    fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl From<&str> for Part {
    fn from(input: &str) -> Part {
        const PARENTHESIS: &[char] = &['{', '}'];
        let values = input.trim_matches(PARENTHESIS).split(",");
        let (mut x, mut m, mut a, mut s) = (0, 0, 0, 0);
        for v in values {
            let (symbol, number) = v.split_once("=").unwrap();
            let number = number.parse::<usize>().unwrap();
            match symbol {
                "x" => x = number,
                "m" => m = number,
                "a" => a = number,
                "s" => s = number,
                _ => panic!("We got something other than xmas"),
            }
        }
        Part { x, m, a, s }
    }
}

pub fn run(input: &str) -> Result<usize, String> {
    let (workflow_str, parts_str) = input.split_once("\n\n").unwrap();

    let workflows: HashMap<String, Workflow> = workflow_str
        .lines()
        .map(|line| {
            let (name, rest) = line.split_once("{").unwrap();
            // let _ = rest.pop();
            let mut rules: Vec<_> = rest[0..(rest.len() - 1)].split(",").collect();
            let default_result = Next::from(rules.pop().unwrap());
            let rules = rules.into_iter().map(Rule::from).collect();
            let workflow = Workflow {
                rules,
                default_result,
            };
            (name.to_string(), workflow)
        })
        .collect();

    let parts: Vec<_> = parts_str.lines().map(Part::from).collect();

    let sum = parts
        .iter()
        .filter(|p| p.run_workflow(workflows.get("in").unwrap(), &workflows))
        .map(Part::sum)
        .sum();

    Ok(sum)
}
