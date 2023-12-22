#![allow(dead_code, unused_variables)]

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
    fn trigger(
        &self,
        modules: &mut HashMap<String, Module>,
        high_count: &mut usize,
        low_count: &mut usize,
    ) -> Vec<Pulse> {
        match self.height {
            PulseHeight::High => *high_count += 1,
            PulseHeight::Low => *low_count += 1,
        }
        eprintln!("{} -{:?}- -> {}", self.source, self.height, self.dest);
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
                            eprintln!("Turned off flipflop {}", self.name);
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

pub fn run(input: &str) -> Result<usize, String> {
    let mut modules: HashMap<String, Module> = input
        .lines()
        .map(Module::from)
        .map(|m| (m.name.clone(), m))
        .collect();

    // Gather all the outputs to add inputs to conjunctions
    // for module in modules.values() {
    //     for output in module.destinations.clone() {
    //         let output: &mut Module = modules.get_mut(&output).unwrap();
    //         if output.spec == ModuleType::Conjunction {
    //             output.inputs.push(module.name.clone());
    //             output.last_input_state.push(PulseHeight::Low);
    //         }
    //     }
    // }

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

    // dbg!(&modules);

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

    let mut pulses: Queue<Pulse> = queue![];
    let mut high_count: usize = 0;
    let mut low_count: usize = 0;

    (0..1000).for_each(|_| {
        let _ = pulses.add(Pulse {
            dest: "broadcaster".to_string(),
            height: PulseHeight::Low,
            source: "button".to_string(),
        });
        while let Ok(pulse) = pulses.remove() {
            let new_pulses = pulse.trigger(&mut modules, &mut high_count, &mut low_count);
            for new in new_pulses {
                let _ = pulses.add(new);
            }
        }
    });
    // dbg!(&modules);
    println!("High: {}, Low: {}", high_count, low_count);
    Ok(high_count * low_count)
}
