use std::ascii::AsciiExt;
use std::collections::BTreeSet;
use iron::typemap::Key;

pub struct WordList;

impl Key for WordList {
    type Value = Vec<String>;
}

pub fn validate_word(s: &str) -> bool {
    s.is_ascii()
    && s.len() > 3
    && s.len() < 8
    && letters_are_unique(s)
}

fn letters_are_unique(s: &str) -> bool {
    let set: BTreeSet<char> = s.chars().collect();
    set.len() == s.len()
}