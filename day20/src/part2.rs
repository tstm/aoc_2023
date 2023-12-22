// use rayon::prelude::*;

use queues::*;
use std::collections::HashMap;

#[derive(Clone)]
struct Pulse {
    dest: String,
    source: String,
    height: PulseHeight,
}

impl Pulse {
    fn trigger(&self, modules: &mut HashMap<String, Module>) -> Vec<Pulse> {
        modules
            .get_mut(&self.dest)
            .unwrap()
            .activate(&self.height, &self.source)
    }
}

#[derive(Debug)]
struct Module {
    destinations: Vec<String>,
    inputs: Vec<String>,
    last_input_state: Vec<PulseHeight>,
    state: bool,
    spec: ModuleType,
    name: String,
}

impl AsRef<Self> for Module {
    fn as_ref(&self) -> &Module {
        &self
    }
}

impl Module {
    fn activate(&mut self, height: &PulseHeight, source: &String) -> Vec<Pulse> {
        match self.spec {
            ModuleType::Broadcast => self
                .destinations
                .iter()
                .map(|dest| Pulse {
                    dest: dest.clone(),
                    height: height.clone(),
                    source: self.name.clone(),
                })
                .collect(),
            ModuleType::Untyped => vec![],
            ModuleType::FlipFlop => {
                if height == &PulseHeight::Low {
                    let new_height = match self.state {
                        true => {
                            self.state = false;
                            PulseHeight::Low
                        }
                        false => {
                            // eprintln!("Turned off flipflop {}", self.name);
                            self.state = true;
                            PulseHeight::High
                        }
                    };
                    self.destinations
                        .iter()
                        .map(|dest| Pulse {
                            dest: dest.clone(),
                            height: new_height.clone(),
                            source: self.name.clone(),
                        })
                        .collect()
                } else {
                    vec![]
                }
            }
            ModuleType::Conjunction => {
                let input_pos = self.inputs.iter().position(|i| i == source).unwrap();
                self.last_input_state[input_pos] = height.clone();
                let new_height = if self
                    .last_input_state
                    .iter()
                    .all(|s| s == &PulseHeight::High)
                {
                    PulseHeight::Low
                } else {
                    PulseHeight::High
                };
                self.destinations
                    .iter()
                    .map(|dest| Pulse {
                        dest: dest.clone(),
                        height: new_height.clone(),
                        source: self.name.clone(),
                    })
                    .collect()
            }
        }
    }
}

impl From<&str> for Module {
    fn from(line: &str) -> Module {
        let (mut module_str, output_str) = line.split_once(" -> ").unwrap();
        let spec = match module_str.chars().nth(0).unwrap() {
            '%' => {
                module_str = &module_str[1..];
                ModuleType::FlipFlop
            }
            '&' => {
                module_str = &module_str[1..];
                ModuleType::Conjunction
            }
            _ => match module_str {
                "broadcaster" => ModuleType::Broadcast,
                _ => ModuleType::Untyped,
            },
        };
        let name = module_str.to_string();
        let destinations = output_str.split(", ").map(|s| s.to_string()).collect();

        Module {
            destinations,
            inputs: vec![],
            last_input_state: vec![],
            state: false,
            spec,
            name,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
enum ModuleType {
    Broadcast,
    Untyped,
    FlipFlop, // A flipflop is off on low and on on high
    Conjunction,
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum PulseHeight {
    High,
    Low,
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let tmp = a;
        a = b;
        b = tmp % b;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

pub fn run(input: &str) -> Result<usize, String> {
    let mut modules: HashMap<String, Module> = input
        .lines()
        .map(Module::from)
        .map(|m| (m.name.clone(), m))
        .collect();

    // Gather all the outputs to add inputs to conjunctions
    let outputs: Vec<(String, String)> = modules
        .values()
        .flat_map(|module| {
            module
                .destinations
                .iter()
                .map(|d| (module.name.clone(), d.clone()))
                .collect::<Vec<_>>()
        })
        .collect();

    outputs.into_iter().for_each(|(source, destination)| {
        if let Some(dest) = modules.get_mut(&destination) {
            if dest.spec == ModuleType::Conjunction {
                dest.inputs.push(source);
                dest.last_input_state.push(PulseHeight::Low);
            }
        } else {
            modules.insert(
                destination.clone(),
                Module {
                    destinations: vec![],
                    inputs: vec![],
                    last_input_state: vec![],
                    state: false,
                    spec: ModuleType::Untyped,
                    name: destination.clone(),
                },
            );
        }
    });

    let before_rx = modules
        .values()
        .find(|m| m.destinations.contains(&"rx".to_string()))
        .unwrap();
    let before_rx_name = before_rx.name.clone();
    let mut tracker = before_rx
        .inputs
        .clone()
        .iter()
        .map(|i| (i.to_string(), None))
        .collect::<HashMap<String, Option<usize>>>();

    let mut pulses: Queue<Pulse> = queue![];

    for presses in 1.. {
        let _ = pulses.add(Pulse {
            dest: "broadcaster".to_string(),
            height: PulseHeight::Low,
            source: "button".to_string(),
        });
        while let Ok(pulse) = pulses.remove() {
            if pulse.dest == before_rx_name && pulse.height == PulseHeight::High {
                *tracker.get_mut(&pulse.source).unwrap() = Some(presses);
                if tracker.values().all(|presses| presses.is_some()) {
                    return Ok(tracker
                        .values()
                        .map(|presses| presses.unwrap())
                        .fold(1, |acc, curr| lcm(acc, curr)));
                }
            }
            let new_pulses = pulse.trigger(&mut modules);
            for new in new_pulses {
                let _ = pulses.add(new);
            }
        }
    }
    Err("Failed.".to_string())
}
