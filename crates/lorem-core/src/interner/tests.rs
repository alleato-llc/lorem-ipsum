use super::*;

#[test]
fn interning_twice_returns_the_same_id() {
    let mut interner = Interner::new();
    let a = interner.intern("lorem");
    let b = interner.intern("lorem");
    assert_eq!(a, b);
}

#[test]
fn distinct_words_get_distinct_ids() {
    let mut interner = Interner::new();
    let a = interner.intern("lorem");
    let b = interner.intern("ipsum");
    assert_ne!(a, b);
}

#[test]
fn resolve_round_trips() {
    let mut interner = Interner::new();
    let id = interner.intern("dolor");
    assert_eq!(interner.resolve(id), "dolor");
}

#[test]
fn lookup_finds_only_interned_words() {
    let mut interner = Interner::new();
    let id = interner.intern("sit");
    assert_eq!(interner.lookup("sit"), Some(id));
    assert_eq!(interner.lookup("amet"), None);
}
