use std::ascii::AsciiExt;
use std::collections::BTreeSet;
use std::cmp::Ordering;
use iron::typemap::Key;
use options::Difficulty;
use ranking::{CommonalityRanker, Ranker};

pub struct WordList;

impl Key for WordList {
    type Value = Vec<String>;
}

pub fn select_by_difficulty(difficulty: Difficulty, word_list: Vec<String>) -> Vec<String> {
    match difficulty {
        Difficulty::Normal => word_list,
        Difficulty::Easy => select(word_list, |a, b| b.cmp(&a)),
        Difficulty::Hard => select(word_list, |a, b| a.cmp(&b)),
    }
}

fn select<F: Fn(i32, i32) -> Ordering>(word_list: Vec<String>, cmp: F) -> Vec<String> {
    let ranker = CommonalityRanker::new(&word_list);
    let mut ranked_word_list: Vec<_> = word_list.iter().map(|word| (word, ranker.score_word(word))).collect();
    
    ranked_word_list.sort_by(|&(_, a), &(_, b)| cmp(a, b));
    
    ranked_word_list.iter().take(word_list.len() / 3).map(|&(word, _)| word.to_owned()).collect()
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