use std::cmp::Ordering;
use std::io::BufRead;
use iron::typemap::Key;
use options::Difficulty;

pub struct WordList;

impl Key for WordList {
    type Value = Vec<String>;
}

pub fn create_word_list<R: BufRead>(reader: &mut R, difficulty: Difficulty) -> Vec<String> {
    use std::ascii::AsciiExt;
    
    let word_list = reader.lines()
        .filter_map(|line| match line {
            Ok(ref line) if line.is_ascii() => Some(line.trim().to_ascii_lowercase()),
            _ => None,
        })
        .filter(|word| validate_word(word))
        .collect();
    
    select_by_difficulty(difficulty, word_list)
}

fn select_by_difficulty(difficulty: Difficulty, word_list: Vec<String>) -> Vec<String> {
    match difficulty {
        Difficulty::Normal => word_list,
        Difficulty::Easy => select(word_list, |a, b| b.cmp(&a)),
        Difficulty::Hard => select(word_list, |a, b| a.cmp(&b)),
    }
}

fn select<F: Fn(i32, i32) -> Ordering>(word_list: Vec<String>, cmp: F) -> Vec<String> {
    use words::{CommonalityRanker, Ranker};
    
    let ranker = CommonalityRanker::new(&word_list);
    let mut ranked_word_list: Vec<_> = word_list.iter().map(|word| (word, ranker.score_word(word))).collect();
    
    ranked_word_list.sort_by(|&(_, a), &(_, b)| cmp(a, b));
    ranked_word_list.iter().take(word_list.len() / 3).map(|&(word, _)| word.to_owned()).collect()
}

fn validate_word(s: &str) -> bool {
    s.len() > 3
    && s.len() < 8
    && letters_are_unique(s)
}

fn letters_are_unique(s: &str) -> bool {
    use std::collections::BTreeSet;
    
    let set: BTreeSet<char> = s.chars().collect();
    set.len() == s.len()
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;
    use options::Difficulty;
    use words;
    
    static RAW_WORD_LIST: &'static str = "Valid\nword\n£ima\nbetter\n";
    
    #[test]
    fn non_ascii_words_are_excluded() {
        let mut reader = get_reader();
        let words = vec![
            "valid",
            "word",
        ];
        
        assert_eq!(&words, &words::create_word_list(&mut reader, Difficulty::Normal))
    }
    
    #[test]
    fn capitalized_words_are_converted_to_lowercase() {
        let mut reader = get_reader();
        let words = vec![
            "valid",
            "word",
        ];
        
        assert_eq!(&words, &words::create_word_list(&mut reader, Difficulty::Normal))
    }
    
    #[test]
    fn words_with_repeat_letters_are_not_allowed() {
        let mut reader = get_reader();
        let words = vec![
            "valid",
            "word",
        ];
        
        assert_eq!(&words, &words::create_word_list(&mut reader, Difficulty::Normal))
    }
    
    fn get_reader<'a>() -> BufReader<&'a [u8]> {
        BufReader::new(RAW_WORD_LIST.as_bytes())
    }
}