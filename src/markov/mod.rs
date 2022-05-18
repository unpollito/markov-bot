pub mod constants;
mod utils;

use self::{
    constants::{
        MARKOV_CHAIN_END, MARKOV_CHAIN_START, RANDOM_END_PROBABILITY, RANDOM_START_PROBABILITY,
    },
    utils::LaxNumber,
};
use rand::{prelude::*, thread_rng};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMarkovChain {
    #[serde(deserialize_with = "LaxNumber::deserialize")]
    pub chat_id: i64,
    // E.g. entries.foo = { bar: 3 } means that word "foo" was followed by word "bar" 3 times
    pub entries: HashMap<String, HashMap<String, u32>>,
}

impl ChatMarkovChain {
    pub fn add_sentence(&mut self, sentence: &str) -> () {
        let sentence = String::from(sentence);
        let words: Vec<String> = sentence
            .split_whitespace()
            .map(|word| word.to_lowercase())
            .collect();
        if words.is_empty() {
            return;
        }

        for i in 0..words.len() + 1 {
            let predecessor = if i == 0 {
                String::from(MARKOV_CHAIN_START)
            } else {
                String::from(&words[i - 1])
            };
            let successor = if i == words.len() {
                String::from(MARKOV_CHAIN_END)
            } else {
                String::from(&words[i])
            };
            self.add_word(&predecessor, &successor);
        }
    }

    fn add_word(&mut self, predecessor: &str, successor: &str) -> () {
        if self.entries.get(predecessor).is_none() {
            self.entries.insert(predecessor.to_string(), HashMap::new());
        }
        let mut predecessor_root_entry = self.entries[predecessor].clone();
        let successor_child_entry = predecessor_root_entry.get(successor);
        let new_val = match successor_child_entry {
            Some(val) => val + 1,
            None => 1,
        };
        predecessor_root_entry.insert(successor.to_string(), new_val);
        self.entries
            .insert(predecessor.to_string(), predecessor_root_entry);

        if self.entries.get(successor).is_none() {
            self.entries.insert(successor.to_string(), HashMap::new());
        }
    }

    pub fn generate_sentence(&self) -> Option<String> {
        if self.entries.len() <= 2 {
            return None;
        }

        let mut rng = thread_rng();
        let initial_word = self.get_initial_word(&mut rng);

        let mut words: Vec<String> = vec![];
        if initial_word != MARKOV_CHAIN_START {
            words.push(initial_word.to_string());
        }

        let mut current_entry = self.entries.get(initial_word).unwrap();
        while !current_entry.is_empty() && rng.gen::<f64>() >= RANDOM_END_PROBABILITY {
            // Each word will be chosen with a probability weighted by the number of times
            // it followed the previous word.
            let mut candidates: Vec<&str> = vec![];
            for (word, num_times) in current_entry.into_iter() {
                for _ in 0..*num_times {
                    candidates.push(word);
                }
            }
            let next_word_index = rng.gen_range(0..candidates.len());
            let next_word = candidates[next_word_index].to_string();
            if next_word != MARKOV_CHAIN_END {
                words.push(next_word.to_string());
            }
            current_entry = self.entries.get(&next_word).unwrap();
        }

        Some(words.join(" "))
    }

    fn get_initial_word(&self, rng: &mut ThreadRng) -> &str {
        if rng.gen::<f64>() < RANDOM_START_PROBABILITY {
            let words: Vec<&String> = self
                .entries
                .keys()
                .filter(|key| *key != MARKOV_CHAIN_START && *key != MARKOV_CHAIN_END)
                .collect();
            let index = rng.gen_range(0..words.len());
            words[index]
        } else {
            MARKOV_CHAIN_START
        }
    }
}
