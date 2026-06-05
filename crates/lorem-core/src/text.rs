//! Pure text helpers: sentence splitting, token normalization, casing.
//! No state, no randomness — every function here is a plain transformation.

/// Split a corpus into raw sentence fragments on terminal punctuation.
pub(crate) fn split_sentences(corpus: &str) -> impl Iterator<Item = &str> {
    corpus.split(|c: char| matches!(c, '.' | '!' | '?' | ';'))
}

/// Lowercase a raw token and strip surrounding punctuation, keeping internal
/// apostrophes and hyphens (e.g. "ship's", "cross-functional").
pub(crate) fn normalize_token(raw: &str) -> Option<String> {
    let cleaned: String = raw
        .chars()
        .filter(|c| c.is_alphabetic() || *c == '\'' || *c == '-')
        .flat_map(|c| c.to_lowercase())
        .collect();
    let cleaned = cleaned.trim_matches(|c| c == '\'' || c == '-');
    if cleaned.is_empty() {
        None
    } else {
        Some(cleaned.to_string())
    }
}

/// Uppercase the first letter, leaving the rest untouched.
pub(crate) fn capitalize_first(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().chain(chars).collect(),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests;
