//! Rendering a walked word sequence into a punctuated sentence string.
//! Pure string assembly — no randomness, no chain knowledge.

use std::collections::HashSet;

use crate::text::capitalize_first;

/// Join words into a sentence: capitalized first word, commas after the
/// marked positions (never on the last word), terminal period.
pub(crate) fn compose_sentence(words: &[&str], commas: &HashSet<usize>) -> String {
    let mut out = String::new();
    for (i, word) in words.iter().enumerate() {
        if i == 0 {
            out.push_str(&capitalize_first(word));
        } else {
            out.push(' ');
            out.push_str(word);
        }
        if i + 1 < words.len() && commas.contains(&i) {
            out.push(',');
        }
    }
    out.push('.');
    out
}

#[cfg(test)]
mod tests;
