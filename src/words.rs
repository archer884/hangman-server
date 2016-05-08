use std::ascii::AsciiExt;
use iron::typemap::Key;
use strsim::levenshtein;

pub struct WordList;

impl Key for WordList {
    type Value = Vec<String>;
}

pub fn rank_word<'a, T: AsRef<str>>(word: &str, word_list: &'a [T]) -> i32 {
    word_list.iter().filter(|witness| levenshtein(witness.as_ref(), word) < 3).count() as i32
}

pub fn validate_word(s: &str) -> bool {
    s.is_ascii() && s.len() > 3 && s.len() < 9
}