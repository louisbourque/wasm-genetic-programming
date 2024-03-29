mod utils;

use rand::prelude::*;
use std::cmp;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

extern crate web_sys;

// A macro to provide `log!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub struct Config {
    pub pop_size: u16,
    pub max_generations: u16,
    pub mutate_prob: f32,
    pub selection: String,
    pub fitness_order: String,
    pub chromosome_function: Vec<String>,
    pub chromosome_terminal: Vec<String>,
    pub chromosome_combined: Vec<String>,
    pub max_fitness_evals: u16,
    pub tree_limit_initial: u16,
    pub tree_limit_running: u16,
}

// #[wasm_bindgen]
// impl Config {
//     #[wasm_bindgen(getter)]
//     pub fn selection(&self) -> String {
//         self.selection.clone()
//     }

//     #[wasm_bindgen(setter)]
//     pub fn set_selection(&mut self, selection: String) {
//         self.selection = selection;
//     }
// }

#[derive(Debug, Clone)]
pub struct Member {
    chromosome: Node,
    fitness: f64,
}

#[wasm_bindgen]
pub struct GP {
    fitness: Vec<[f64; 2]>,
    config: Config,
    fitness_evaluations: usize,
    gen: u16,
    population: Vec<Member>,
    done: bool,
}

#[derive(Debug, Clone)]
enum Action {
    Function(String),
    Terminal(f64),
    X,
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
struct Node {
    action: Action,
    arg1: Option<Box<Node>>,
    arg2: Option<Box<Node>>,
}

#[wasm_bindgen]
impl GP {
    pub fn new(fitness_array: Vec<f64>) -> Self {
        let mut fitness: Vec<[f64; 2]> = Vec::new();
        let mut pair: [f64; 2] = [0.0, 0.0];
        for (index, value) in fitness_array.iter().enumerate() {
            pair[index % 2] = *value;
            if index % 2 == 1 {
                fitness.push(pair);
                pair = [0.0, 0.0];
            }
        }

        Self {
            fitness,
            config: Config {
                pop_size: 4000,
                max_generations: 51,
                mutate_prob: 0.02,
                selection: "tournament".to_string(),
                fitness_order: "desc".to_string(),
                chromosome_function: ["+", "-", "*", "/", "sin", "cos", "exp"]
                    .iter()
                    .map(|x| x.to_string())
                    .collect(),
                chromosome_terminal: vec!["x".to_string(), "R".to_string()],
                chromosome_combined: ["+", "-", "*", "/", "sin", "cos", "exp", "x", "R"]
                    .iter()
                    .map(|x| x.to_string())
                    .collect(),
                max_fitness_evals: 20000,
                tree_limit_initial: 6,
                tree_limit_running: 17,
            },
            fitness_evaluations: 0,
            gen: 0,
            population: Vec::new(),
            done: false,
        }
    }

    pub fn init(&mut self) {
        utils::set_panic_hook();
        //make initial random population
        let mut population: Vec<Member> = Vec::new();
        let mut i = 0;
        loop {
            //call generate_chromosome with half false, half true (for Ramped Half and Half)
            let chromosome = self.generate_chromosome(i > self.config.pop_size / 2);
            let member = Member {
                fitness: self.measure_fitness(&chromosome),
                chromosome,
            };
            if self.insert_into_population(member, &mut population) {
                i += 1;
            }

            if i >= self.config.pop_size {
                break;
            }
        }
        self.population = population;
    }

    pub fn tick(&mut self) -> String {
        utils::set_panic_hook();
        let mut rng = rand::thread_rng();
        self.gen += 1;

        //termination sat for run?
        //if best solution has a fitness less than 0.001, we can stop
        if self.config.max_generations <= self.gen
            || self.population.last().unwrap().fitness < 0.001
            || self.done
        {
            log!("RUN COMPLETED ======================");
            log!("Fitness Evaluations: {}", self.fitness_evaluations);
            self.done = true;
            let mut best_member_fitness = 0.0;
            let mut best_member_string = String::new();
            let mut best_member_json = String::new();
            if let Some(member) = self.population.last() {
                best_member_fitness = member.fitness;
                best_member_string = self.chromosome_to_string(&member.chromosome);
                best_member_json = self.chromosome_to_json(&member.chromosome);
            }
            return format!(
                "{{\"done\": {}, \"fitness\":{} ,\"best\":\"{}\",\"gen\":\"{}\", \"chromosome\":{}}}",
                true, best_member_fitness, best_member_string, self.gen, best_member_json
            );
        }

        let mut new_population: Vec<Member> = Vec::new();
        let mut i = 0;
        while i < self.population.len() {
            let rnum: f64 = rng.gen();
            //10% chance of reproduction, 90% chance of crossover
            //select one individual based on fitness
            let individual1 = self.select_from_population(&self.population);
            if rnum > 0.9 {
                //insert copy in new pop
                if self.insert_into_population(individual1, &mut new_population) {
                    i += 1;
                }
            } else {
                //select two individuals based on fitness
                let individual2 = self.select_from_population(&self.population);
                //perform crossover
                let chromosome1 =
                    self.crossover_function(&individual1.chromosome, &individual2.chromosome);
                let child1 = Member {
                    fitness: self.measure_fitness(&chromosome1),
                    chromosome: chromosome1,
                };
                let chromosome2 =
                    self.crossover_function(&individual1.chromosome, &individual2.chromosome);
                let child2 = Member {
                    fitness: self.measure_fitness(&chromosome2),
                    chromosome: chromosome2,
                };

                let mut candidates: Vec<Member> = vec![individual1, individual2, child1, child2];
                if self.config.fitness_order == "desc" {
                    candidates.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
                } else {
                    candidates.sort_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap());
                }
                if self.insert_into_population(candidates.pop().unwrap(), &mut new_population) {
                    i += 1;
                }
                if self.insert_into_population(candidates.pop().unwrap(), &mut new_population) {
                    i += 1;
                }
            }
        }
        //if new_population is shorter than population, then we hit a stop condition. The best solution
        // may be in population, fill new population with the best of population so they are of the same size
        while new_population.len() < self.population.len() {
            new_population.push(self.population.remove(0))
        }
        self.population = new_population;

        //sort population by fitness
        if self.config.fitness_order == "desc" {
            self.population
                .sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
        } else {
            self.population
                .sort_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap());
        }

        //return best so far
        let mut best_member_fitness = 0.0;
        let mut best_member_string = String::new();
        let mut best_member_json = String::new();
        if let Some(member) = self.population.last() {
            best_member_fitness = member.fitness;
            best_member_string = self.chromosome_to_string(&member.chromosome);
            best_member_json = self.chromosome_to_json(&member.chromosome);
        }
        return format!(
            "{{\"done\": {}, \"fitness\":{} ,\"best\":\"{}\",\"gen\":\"{}\", \"chromosome\":{}}}",
            false, best_member_fitness, best_member_string, self.gen, best_member_json
        );
    }
}

impl GP {
    fn generate_chromosome(&self, grow: bool) -> Node {
        self.generate_chromosome_recursive(self.config.tree_limit_initial, grow)
    }

    fn generate_chromosome_recursive(&self, limit: u16, grow: bool) -> Node {
        let available_chromosomes = if grow {
            &self.config.chromosome_function
        } else if limit <= 1 {
            &self.config.chromosome_terminal
        } else {
            &self.config.chromosome_combined
        };
        let max_action_index = available_chromosomes.len();
        let mut rng = rand::thread_rng();
        let random_number: f64 = rng.gen();
        let action_index = (max_action_index as f64 * random_number).floor() as usize;
        let action = available_chromosomes.get(action_index).unwrap();
        if action == "R" {
            let random_terminal: f64 = rng.gen();
            Node {
                action: Action::Terminal((random_terminal * 10.0).ceil()),
                arg1: None,
                arg2: None,
            }
        } else if action == "x" {
            Node {
                action: Action::X,
                arg1: None,
                arg2: None,
            }
        } else {
            Node {
                action: Action::Function(action.to_string()),
                arg1: Some(Box::new(
                    self.generate_chromosome_recursive(limit - 1, false),
                )),
                arg2: Some(Box::new(
                    self.generate_chromosome_recursive(limit - 1, false),
                )),
            }
        }
    }

    fn insert_into_population(&self, member: Member, population: &mut Vec<Member>) -> bool {
        population.push(member);
        true
    }

    fn measure_fitness(&mut self, chromosome: &Node) -> f64 {
        self.fitness_evaluations += 1;
        let mut fitness: f64 = 0.0;
        let mut x: f64;

        for x_y in self.fitness.as_slice() {
            x = x_y[0];
            let eval_res = self.eval_tree(chromosome, x);
            let abs = (x_y[1] - eval_res).abs();
            fitness += abs;
        }
        if fitness.is_nan() {
            if self.config.fitness_order == "desc" {
                return 9999999.0;
            } else {
                return 0.0;
            }
        }
        fitness
    }

    fn eval_tree(&self, chromosome: &Node, x: f64) -> f64 {
        match &chromosome.action {
            Action::Function(function_name) => {
                if let Some(arg1) = &chromosome.arg1 {
                    if let Some(arg2) = &chromosome.arg2 {
                        return match function_name.as_str() {
                            "+" => self.eval_tree(arg1, x) + self.eval_tree(arg2, x),
                            "-" => self.eval_tree(arg1, x) - self.eval_tree(arg2, x),
                            "*" => self.eval_tree(arg1, x) * self.eval_tree(arg2, x),
                            "/" => self.eval_tree(arg1, x) / self.eval_tree(arg2, x),
                            "sin" => (self.eval_tree(arg1, x)).sin(),
                            "cos" => (self.eval_tree(arg1, x)).cos(),
                            "exp" => (self.eval_tree(arg1, x)).powf(self.eval_tree(arg2, x)),
                            _ => 9999999.0,
                        };
                    }
                }
                9999999.0
            }
            Action::Terminal(number) => *number,
            Action::X => x,
        }
    }

    fn select_from_population(&self, population: &[Member]) -> Member {
        let mut rng = rand::thread_rng();
        let mut choices: Vec<&Member> = Vec::new();
        let max_population_index = population.len();
        for _ in 0..6 {
            if let Some(member) =
                population.get((max_population_index as f64 * rng.gen::<f64>()).floor() as usize)
            {
                choices.push(member);
            }
        }
        let r: f64 = rng.gen();

        if r < 0.5 {
            return choices
                .remove((choices.len() as f64 * rng.gen::<f64>()).floor() as usize)
                .clone();
        }
        if self.config.fitness_order == "desc" {
            choices.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
        } else {
            choices.sort_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap());
        }
        choices.pop().unwrap().clone()
    }

    fn crossover_function(&self, parent1: &Node, parent2: &Node) -> Node {
        let picked_node = self.pick_node(parent2);
        let mut new_parent = parent1.clone();
        self.swap_node(&mut new_parent, &picked_node);

        if self.count_node_depth(&new_parent) > self.config.tree_limit_running {
            return parent1.clone();
        }
        new_parent
    }

    fn pick_node(&self, node: &Node) -> Node {
        let mut rng = rand::thread_rng();
        let number_of_nodes = self.count_nodes(node);
        let random_number: usize = (number_of_nodes as f64 * rng.gen::<f64>()).floor() as usize;
        let (picked_node, _) = self.traverse_tree(node, random_number);
        picked_node.unwrap_or_else(|| node.clone())
    }

    fn traverse_tree(&self, node: &Node, counter: usize) -> (Option<Node>, usize) {
        if counter == 0 {
            return (Some(node.clone()), 0);
        }
        if node.arg1.is_none() || node.arg2.is_none() {
            return (None, counter);
        }
        if let Some(arg1) = &node.arg1 {
            let (arg1_node, counter) = self.traverse_tree(arg1, counter - 1);
            if arg1_node.is_some() {
                return (arg1_node, counter);
            }
            if let Some(arg2) = &node.arg2 {
                self.traverse_tree(arg2, counter - 1)
            } else {
                (None, counter)
            }
        } else {
            (None, counter)
        }
    }

    fn swap_node(&self, parent_node: &mut Node, new_node: &Node) {
        let mut rng = rand::thread_rng();
        let number_of_nodes = self.count_nodes(parent_node);
        let random_number: usize = (number_of_nodes as f64 * rng.gen::<f64>()).floor() as usize;
        self.update_tree(parent_node, new_node, random_number);
    }

    fn update_tree(&self, node: &mut Node, new_node: &Node, counter: usize) -> usize {
        if counter == 0 {
            node.action = new_node.action.clone();
            node.arg1 = new_node.arg1.clone();
            node.arg2 = new_node.arg2.clone();
            return 0;
        }
        if node.arg1.is_none() || node.arg2.is_none() {
            return counter;
        }
        if let Some(ref mut arg1) = node.arg1 {
            let counter = self.update_tree(arg1.as_mut(), new_node, counter - 1);
            if counter == 0 {
                return 0;
            }
            if let Some(ref mut arg2) = node.arg2 {
                self.update_tree(arg2.as_mut(), new_node, counter - 1)
            } else {
                counter
            }
        } else {
            counter
        }
    }

    fn count_nodes(&self, node: &Node) -> usize {
        if let Some(arg1) = &node.arg1 {
            if let Some(arg2) = &node.arg2 {
                return self.count_nodes(arg1) + self.count_nodes(arg2) + 1;
            }
        }
        1
    }

    fn count_node_depth(&self, node: &Node) -> u16 {
        if let Some(arg1) = &node.arg1 {
            if let Some(arg2) = &node.arg2 {
                return cmp::max(self.count_node_depth(arg1), self.count_node_depth(arg2)) + 1;
            }
        }
        1
    }

    fn chromosome_to_string(&self, chromosome: &Node) -> String {
        match &chromosome.action {
            Action::Function(function_name) => {
                if let Some(arg1) = &chromosome.arg1 {
                    if let Some(arg2) = &chromosome.arg2 {
                        return match function_name.as_str() {
                            "+" => format!(
                                "({} + {})",
                                self.chromosome_to_string(arg1),
                                self.chromosome_to_string(arg2)
                            ),
                            "-" => format!(
                                "({} - {})",
                                self.chromosome_to_string(arg1),
                                self.chromosome_to_string(arg2)
                            ),
                            "*" => format!(
                                "({} * {})",
                                self.chromosome_to_string(arg1),
                                self.chromosome_to_string(arg2)
                            ),
                            "/" => format!(
                                "({} / {})",
                                self.chromosome_to_string(arg1),
                                self.chromosome_to_string(arg2)
                            ),
                            "sin" => format!("sin({})", self.chromosome_to_string(arg1)),

                            "cos" => format!("cos({})", self.chromosome_to_string(arg1)),

                            "exp" => format!(
                                "exp({}, {})",
                                self.chromosome_to_string(arg1),
                                self.chromosome_to_string(arg2)
                            ),

                            _ => "".to_string(),
                        };
                    }
                }
                "(error)".to_string()
            }
            Action::Terminal(number) => number.to_string(),
            Action::X => "x".to_string(),
        }
    }

    fn chromosome_to_json(&self, chromosome: &Node) -> String {
        match &chromosome.action {
            Action::Function(function_name) => {
                if let Some(arg1) = &chromosome.arg1 {
                    if let Some(arg2) = &chromosome.arg2 {
                        return match function_name.as_str() {
                            "+" => format!(
                                "{{\"action\":\"+\",\"arg1\":{},\"arg2\":{}}}",
                                self.chromosome_to_json(arg1),
                                self.chromosome_to_json(arg2)
                            ),
                            "-" => format!(
                                "{{\"action\":\"-\",\"arg1\":{},\"arg2\":{}}}",
                                self.chromosome_to_json(arg1),
                                self.chromosome_to_json(arg2)
                            ),
                            "*" => format!(
                                "{{\"action\":\"*\",\"arg1\":{},\"arg2\":{}}}",
                                self.chromosome_to_json(arg1),
                                self.chromosome_to_json(arg2)
                            ),
                            "/" => format!(
                                "{{\"action\":\"/\",\"arg1\":{},\"arg2\":{}}}",
                                self.chromosome_to_json(arg1),
                                self.chromosome_to_json(arg2)
                            ),
                            "sin" => format!(
                                "{{\"action\":\"sin\",\"arg1\":{}}}",
                                self.chromosome_to_json(arg1)
                            ),

                            "cos" => format!(
                                "{{\"action\":\"cos\",\"arg1\":{}}}",
                                self.chromosome_to_json(arg1)
                            ),

                            "exp" => format!(
                                "{{\"action\":\"exp\",\"arg1\":{},\"arg2\":{}}}",
                                self.chromosome_to_json(arg1),
                                self.chromosome_to_json(arg2)
                            ),

                            _ => "".to_string(),
                        };
                    }
                }
                "(error)".to_string()
            }
            Action::Terminal(number) => format!("{{\"action\":{}}}", number),
            Action::X => "{\"action\":\"x\"}".to_string(),
        }
    }
}
