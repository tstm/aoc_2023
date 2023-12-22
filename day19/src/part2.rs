#![allow(dead_code, unused_variables)]

// use rayon::prelude::*;

use std::{
    collections::HashMap,
    ops::{Range, RangeInclusive},
};

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

#[derive(PartialEq, Eq, Debug, Clone)]
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

#[derive(Debug)]
struct PartRange {
    x: RangeInclusive<usize>,
    m: RangeInclusive<usize>,
    a: RangeInclusive<usize>,
    s: RangeInclusive<usize>,
    status: Option<Next>,
}

impl PartRange {
    fn run(self, workflows: &HashMap<String, Workflow>) -> Vec<PartRange> {
        let mut unresolved = vec![self];
        let mut resolved = vec![];
        while let Some(pr) = unresolved.pop() {
            match pr.status {
                Some(Next::Accept) | Some(Next::Reject) => resolved.push(pr),
                Some(Next::Forward(_)) => {
                    let mut splits = pr.split_workflow(workflows);
                    unresolved.append(&mut splits);
                }
                None => panic!("There should always be a status"),
            }
        }
        resolved
    }

    fn combinations(self) -> usize {
        self.x.count() * self.m.count() * self.a.count() * self.s.count()
    }

    fn split_workflow(self, workflows: &HashMap<String, Workflow>) -> Vec<PartRange> {
        let mut resolved = vec![];
        let workflow = match self.status.clone() {
            Some(Next::Forward(id)) => workflows.get(&id).unwrap(),
            _ => panic!("Only forward workflows here"),
        };
        let mut next = self;

        for rule in workflow.rules.iter() {
            let (arm1, arm2) = next.split(&rule);
            next = arm2;
            resolved.push(arm1);
        }
        next.status = Some(workflow.default_result.clone());
        resolved.push(next);

        resolved
    }

    fn split(&self, rule: &Rule) -> (PartRange, PartRange) {
        let (mut arm1, mut arm2) = match rule.symbol {
            'x' => (
                PartRange {
                    x: (*self.x.start())..=(rule.limit - 1),
                    m: self.m.clone(),
                    a: self.a.clone(),
                    s: self.s.clone(),
                    status: None,
                },
                PartRange {
                    x: rule.limit..=*self.x.end(),
                    m: self.m.clone(),
                    a: self.a.clone(),
                    s: self.s.clone(),
                    status: None,
                },
            ),
            'm' => (
                PartRange {
                    m: (*self.m.start())..=(rule.limit - 1),
                    x: self.x.clone(),
                    a: self.a.clone(),
                    s: self.s.clone(),
                    status: None,
                },
                PartRange {
                    m: rule.limit..=*self.m.end(),
                    x: self.x.clone(),
                    a: self.a.clone(),
                    s: self.s.clone(),
                    status: None,
                },
            ),
            'a' => (
                PartRange {
                    a: (*self.a.start())..=(rule.limit - 1),
                    x: self.x.clone(),
                    m: self.m.clone(),
                    s: self.s.clone(),
                    status: None,
                },
                PartRange {
                    a: rule.limit..=*self.a.end(),
                    x: self.x.clone(),
                    m: self.m.clone(),
                    s: self.s.clone(),
                    status: None,
                },
            ),
            's' => (
                PartRange {
                    s: (*self.s.start())..=(rule.limit - 1),
                    x: self.x.clone(),
                    m: self.m.clone(),
                    a: self.a.clone(),
                    status: None,
                },
                PartRange {
                    s: (rule.limit)..=*self.s.end(),
                    x: self.x.clone(),
                    m: self.m.clone(),
                    a: self.a.clone(),
                    status: None,
                },
            ),
            _ => panic!("There should not be more symbols"),
        };
        match rule.op {
            Operation::GreaterThan => {
                arm2.status = Some(rule.result.clone());
                (arm2, arm1)
            }
            Operation::LessThan => {
                arm1.status = Some(rule.result.clone());
                (arm1, arm2)
            }
        }
    }
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
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

    let part_range = PartRange {
        x: 1..=4000,
        m: 1..=4000,
        a: 1..=4000,
        s: 1..=4000,
        status: Some(Next::Forward("in".to_string())),
    };

    let ranges = part_range.run(&workflows);
    dbg!(&ranges);

    let sum = ranges
        .into_iter()
        .filter(|r| r.status == Some(Next::Accept))
        .map(PartRange::combinations)
        .sum();

    // let parts: Vec<_> = parts_str.lines().map(Part::from).collect();

    // let sum = parts
    //     .iter()
    //     .filter(|p| p.run_workflow(workflows.get("in").unwrap(), &workflows))
    //     .map(Part::sum)
    //     .sum();

    Ok(sum)
}
