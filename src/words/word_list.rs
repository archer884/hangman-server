use std::cmp::Ordering;
use std::io::BufRead;
use iron::typemap::Key;
use options::Difficulty;

pub struct WordList;

impl Key for WordList {
    type Value = WordService;
}

pub struct WordService {
    easy: Vec<String>,
    normal: Vec<String>,
    hard: Vec<String>,
}

impl WordService {
    pub fn create<R: BufRead>(reader: &mut R) -> WordService {
        use std::ascii::AsciiExt;
        
        let words: Vec<_> = reader.lines()
            .filter_map(|line| match line {
                Ok(ref line) if line.is_ascii() => Some(line.trim().to_ascii_lowercase()),
                _ => None,
            })
            .filter(|word| validate_word(word))
            .collect();
            
        WordService {
            easy: select(&words, |a, b| b.cmp(&a)),
            hard: select(&words, |a, b| a.cmp(&b)),
            normal: words,
        }
    }
    
    pub fn empty() -> WordService {
        WordService {
            easy: vec![],
            normal: vec![],
            hard: vec![],
        }
    }
}

fn select<F: Fn(i32, i32) -> Ordering>(word_list: &[String], cmp: F) -> Vec<String> {
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