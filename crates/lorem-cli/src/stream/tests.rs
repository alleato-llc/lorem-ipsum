use super::*;

#[test]
fn first_word_gets_no_separator() {
    let mut wrap = LineWrap::new(10);
    assert_eq!(wrap.separator(5), "");
}

#[test]
fn words_within_budget_get_spaces() {
    let mut wrap = LineWrap::new(10);
    wrap.separator(4); // "word"
    assert_eq!(wrap.separator(4), " "); // "word word" = 9 ≤ 10
}

#[test]
fn overflow_breaks_the_line() {
    let mut wrap = LineWrap::new(10);
    wrap.separator(4);
    wrap.separator(4); // line now 9 wide
    assert_eq!(wrap.separator(3), "\n"); // 9 + 1 + 3 > 10
}

#[test]
fn budget_resets_after_a_break() {
    let mut wrap = LineWrap::new(10);
    wrap.separator(8);
    assert_eq!(wrap.separator(8), "\n");
    assert_eq!(wrap.separator(1), " "); // 8 + 1 + 1 = 10 fits
}

#[test]
fn oversized_word_still_lands_on_its_own_line() {
    let mut wrap = LineWrap::new(5);
    wrap.separator(3);
    assert_eq!(wrap.separator(20), "\n");
    assert_eq!(wrap.separator(20), "\n");
}
