use super::*;

#[test]
fn splits_on_terminal_punctuation() {
    let parts: Vec<&str> = split_sentences("One two. Three four! Five six? Seven; eight").collect();
    assert_eq!(parts, ["One two", " Three four", " Five six", " Seven", " eight"]);
}

#[test]
fn normalize_lowercases_and_strips_punctuation() {
    assert_eq!(normalize_token("Hello,"), Some("hello".to_string()));
    assert_eq!(normalize_token("\"Quoted\""), Some("quoted".to_string()));
    assert_eq!(normalize_token("WORLD."), Some("world".to_string()));
}

#[test]
fn normalize_keeps_internal_apostrophes_and_hyphens() {
    assert_eq!(normalize_token("ship's"), Some("ship's".to_string()));
    assert_eq!(
        normalize_token("cross-functional"),
        Some("cross-functional".to_string())
    );
}

#[test]
fn normalize_trims_edge_apostrophes_and_hyphens() {
    assert_eq!(normalize_token("'tis'"), Some("tis".to_string()));
    assert_eq!(normalize_token("-dash-"), Some("dash".to_string()));
}

#[test]
fn normalize_rejects_non_words() {
    assert_eq!(normalize_token("123"), None);
    assert_eq!(normalize_token("--"), None);
    assert_eq!(normalize_token(""), None);
}

#[test]
fn capitalizes_first_letter_only() {
    assert_eq!(capitalize_first("lorem"), "Lorem");
    assert_eq!(capitalize_first("ipsum dolor"), "Ipsum dolor");
    assert_eq!(capitalize_first(""), "");
}
