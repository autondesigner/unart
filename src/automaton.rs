use crate::cell::*;
use crate::rule_key::*;
use crate::tape::*;
use rand::prelude::*;
use std::collections::HashMap;

pub struct Automaton {
    pub size: usize,
    pub states_count: usize,
    pub tape: Tape<Cell>,
    pub back_tape: Tape<Cell>,
    pub rule: HashMap<RuleKey, usize>,
}

impl Automaton {
    pub fn new(rng: &mut StdRng, size: usize, states_count: usize) -> Automaton {
        let tape = Automaton::build_tape(rng, size, states_count);
        let back_tape = Tape::<Cell>::new(size);
        let rule = Automaton::build_rule(rng, states_count);
        Automaton {
            size,
            states_count,
            tape,
            back_tape,
            rule,
        }
    }
    pub fn build_tape(rng: &mut StdRng, size: usize, states_count: usize) -> Tape<Cell> {
        let mut tape = Tape::<Cell>::new(size);
        for i in 0..size {
            tape.cells[i].level = rng.gen_range(0..states_count);
        }
        tape
    }
    pub fn fill_back_tape(&mut self) {
        for i in 0..self.size {
            self.back_tape.cells[i].level = self.tape.cells[i].level;
        }
    }
    pub fn get_left_index(&self, index: usize) -> usize {
        if index == 0 {
            return self.size - 1;
        }
        index - 1
    }
    pub fn get_right_index(&self, index: usize) -> usize {
        if index == self.size - 1 {
            return 0;
        }
        index + 1
    }
    pub fn get_neighboorhood(&self, index: usize) -> [usize; 3] {
        let a = self.back_tape.cells[self.get_left_index(index)].level;
        let b = self.back_tape.cells[index].level;
        let c = self.back_tape.cells[self.get_right_index(index)].level;
        [a, b, c]
    }
    pub fn apply_rule(&self, neighborhood: [usize; 3]) -> usize {
        self.rule[&neighborhood]
    }
    pub fn iterate(&mut self) {
        self.fill_back_tape();
        for i in 0..self.size {
            let neighborhood = self.get_neighboorhood(i);
            self.tape.cells[i].level = self.apply_rule(neighborhood);
        }
    }
    pub fn build_rule(rng: &mut StdRng, states_count: usize) -> HashMap<RuleKey, usize> {
        let mut rule = HashMap::<RuleKey, usize>::new();
        for i in 0..states_count {
            for j in 0..states_count {
                for k in 0..states_count {
                    rule.insert([i, j, k], rng.gen_range(0..states_count));
                }
            }
        }
        rule
    }
}
