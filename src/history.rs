use std::collections::HashMap;

use crate::client::Hashable;

pub struct HistManager<T: Hashable + Clone> {
    hist: HashMap<u64, T>,
}

impl<T: Hashable + Clone> HistManager<T> {
    pub fn new() -> Self {
        HistManager {
            hist: HashMap::new(),
        }
    }

    pub fn add(&mut self, value: T) {
        let hash = value.hash_value();
        self.hist.entry(hash).or_insert(value);
    }

    pub fn add_vec(&mut self, values: &Vec<T>) {
        for value in values {
            self.add(value.clone());
        }
    }

    pub fn clear(&mut self) {
        self.hist.clear()
    }

    pub fn histories(&self, current: &[T]) -> Vec<T> {
        let current_h = current.iter().map(|c| c.hash_value()).collect::<Vec<u64>>();

        self.hist
            .iter()
            .filter(|(hist_key, _)| !current_h.iter().any(|c| c == *hist_key))
            .map(|(_, value)| value.clone())
            .collect()
    }
}
