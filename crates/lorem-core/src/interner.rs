//! Word interning: a bidirectional word ↔ `u32` id table that keeps the
//! Markov transition table compact.

use std::collections::HashMap;

#[derive(Default)]
pub(crate) struct Interner {
    words: Vec<String>,
    index: HashMap<String, u32>,
}

impl Interner {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Id for `word`, allocating one on first sight.
    pub(crate) fn intern(&mut self, word: &str) -> u32 {
        if let Some(&id) = self.index.get(word) {
            return id;
        }
        let id = self.words.len() as u32;
        self.words.push(word.to_string());
        self.index.insert(word.to_string(), id);
        id
    }

    /// The word behind `id`. Panics on an id this interner never issued.
    pub(crate) fn resolve(&self, id: u32) -> &str {
        &self.words[id as usize]
    }

    /// Id for `word` if it has been interned.
    pub(crate) fn lookup(&self, word: &str) -> Option<u32> {
        self.index.get(word).copied()
    }
}

#[cfg(test)]
mod tests;
