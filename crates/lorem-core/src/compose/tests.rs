use super::*;

#[test]
fn capitalizes_joins_and_terminates() {
    let words = ["lorem", "ipsum", "dolor"];
    assert_eq!(
        compose_sentence(&words, &HashSet::new()),
        "Lorem ipsum dolor."
    );
}

#[test]
fn places_commas_after_marked_positions() {
    let words = ["lorem", "ipsum", "dolor", "sit"];
    let commas = HashSet::from([1]);
    assert_eq!(
        compose_sentence(&words, &commas),
        "Lorem ipsum, dolor sit."
    );
}

#[test]
fn never_puts_a_comma_on_the_last_word() {
    let words = ["lorem", "ipsum"];
    let commas = HashSet::from([1]);
    assert_eq!(compose_sentence(&words, &commas), "Lorem ipsum.");
}

#[test]
fn single_word_sentence() {
    assert_eq!(compose_sentence(&["lorem"], &HashSet::new()), "Lorem.");
}
