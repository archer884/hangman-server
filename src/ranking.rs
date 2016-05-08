use std::collections::BTreeMap;

pub trait Ranker {
    fn score_word(&self, word: &str) -> i32;
}

pub struct CommonalityRanker {
    scores: BTreeMap<char, i32>
}

impl CommonalityRanker {
    pub fn new<T: AsRef<str>>(words: &[T]) -> CommonalityRanker {
        let ranked_letters = words.iter().flat_map(|word| word.as_ref().chars())
            .fold(BTreeMap::new(), |mut map, letter| {
                *map.entry(letter).or_insert(0) += 1;
                map
            });

        let mut ranked_letters: Vec<_> = ranked_letters.into_iter().collect();
        ranked_letters.sort_by(|&(_, a), &(_, b)| a.cmp(&b));

        CommonalityRanker {
            scores: ranked_letters.into_iter()
                .enumerate()
                .map(|(idx, (letter, _))| (letter, idx as i32))
                .collect()
        }
    }
}

impl Ranker for CommonalityRanker {
    fn score_word(&self, word: &str) -> i32 {
        use std::ops::Add;

        word.chars().map(|c| 
            self.scores.get(&c).map(|&score| score).unwrap_or(0)
        ).fold(0, Add::add)
    }
}