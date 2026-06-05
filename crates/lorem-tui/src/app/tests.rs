use super::*;

/// A test app decoupled from any settings file on disk.
fn app() -> App {
    App::from_options(GeneratorOptions::default())
}

#[test]
fn navigation_stays_within_field_bounds() {
    let mut app = app();
    app.on_key(KeyCode::Up);
    assert_eq!(app.field, 0);
    for _ in 0..FIELDS.len() + 5 {
        app.on_key(KeyCode::Down);
    }
    assert_eq!(app.field, FIELDS.len() - 1);
}

#[test]
fn theme_cycles_in_both_directions() {
    let mut app = app();
    app.on_key(KeyCode::Left);
    assert_eq!(app.theme_idx, Theme::ALL.len() - 1, "should wrap backwards");
    app.on_key(KeyCode::Right);
    assert_eq!(app.theme_idx, 0);
}

#[test]
fn count_steps_by_ten_in_words_mode() {
    let mut app = app();
    app.on_key(KeyCode::Down); // Mode
    app.on_key(KeyCode::Right); // Paragraphs → Sentences
    app.on_key(KeyCode::Right); // Sentences → Words
    assert_eq!(app.mode(), Mode::Words);
    app.on_key(KeyCode::Down); // Count
    let before = app.count;
    app.on_key(KeyCode::Right);
    assert_eq!(app.count, before + 10);
}

#[test]
fn count_clamps_to_mode_maximum() {
    let mut app = app();
    app.field = 2;
    for _ in 0..1000 {
        app.on_key(KeyCode::Right);
    }
    assert_eq!(app.count, app.count_max());
    for _ in 0..1000 {
        app.on_key(KeyCode::Left);
    }
    assert_eq!(app.count, 1);
}

#[test]
fn sentence_fields_dim_outside_paragraphs_mode() {
    let mut app = app();
    assert!(app.field_active(3));
    assert!(app.field_active(4));
    app.field = 1;
    app.on_key(KeyCode::Right); // → Sentences
    assert!(!app.field_active(3));
    assert!(!app.field_active(4));
    assert!(app.field_active(5), "word range applies in sentences mode");
    app.on_key(KeyCode::Right); // → Words
    assert!(!app.field_active(5));
    assert!(!app.field_active(6));
}

#[test]
fn opener_field_dims_for_non_classic_themes() {
    let mut app = app();
    assert!(app.field_active(OPENER_FIELD));
    app.on_key(KeyCode::Right); // Theme: Classic → next
    assert!(!app.field_active(OPENER_FIELD));
}

#[test]
fn opener_toggles_with_arrows() {
    let mut app = app();
    app.field = OPENER_FIELD;
    assert!(app.start_with_lorem);
    app.on_key(KeyCode::Right);
    assert!(!app.start_with_lorem);
    app.on_key(KeyCode::Left);
    assert!(app.start_with_lorem);
}

#[test]
fn seed_field_accepts_only_digits() {
    let mut app = app();
    app.field = SEED_FIELD;
    for c in ['4', '2', 'x', '!', '7'] {
        app.on_key(KeyCode::Char(c));
    }
    assert_eq!(app.seed_input, "427");
    app.on_key(KeyCode::Backspace);
    assert_eq!(app.seed_input, "42");
    assert_eq!(app.options().seed, Some(42));
}

#[test]
fn tab_toggles_focus() {
    let mut app = app();
    assert!(app.focus == Focus::Form);
    app.on_key(KeyCode::Tab);
    assert!(app.focus == Focus::Output);
    app.on_key(KeyCode::Tab);
    assert!(app.focus == Focus::Form);
}

#[test]
fn q_and_esc_quit() {
    let mut app = app();
    app.on_key(KeyCode::Char('q'));
    assert!(app.quit);
    let mut app2 = app;
    app2.quit = false;
    app2.on_key(KeyCode::Esc);
    assert!(app2.quit);
}

#[test]
fn enter_generates_with_form_options() {
    let mut app = app();
    app.field = SEED_FIELD;
    for c in ['9', '9'] {
        app.on_key(KeyCode::Char(c));
    }
    app.on_key(KeyCode::Enter);
    let result = app.result.as_ref().unwrap();
    assert_eq!(result.seed, 99);
    assert_eq!(app.table_state.selected(), Some(0));
}

#[test]
fn output_navigation_clamps_to_item_range() {
    let mut app = app();
    app.on_key(KeyCode::Enter); // generate (3 paragraphs)
    app.on_key(KeyCode::Tab); // focus output
    let len = app.result.as_ref().unwrap().items.len();
    for _ in 0..len + 5 {
        app.on_key(KeyCode::Down);
    }
    assert_eq!(app.table_state.selected(), Some(len - 1));
    app.on_key(KeyCode::Home);
    assert_eq!(app.table_state.selected(), Some(0));
}

#[test]
fn min_max_ordering_is_clamped_in_options() {
    let mut app = app();
    app.min_sentences = 9;
    app.max_sentences = 2;
    let opts = app.options();
    assert!(opts.max_sentences >= opts.min_sentences);
}
