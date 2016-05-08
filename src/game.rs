use std::collections::{HashMap, BTreeSet};
use iron::typemap::Key;
use model::GameStateModel;
use outcome::Outcome;

pub struct Game {
    word: String,
    guesses: BTreeSet<char>,
    strike_count: i32,
}

impl Game {
    pub fn new<T: Into<String>>(word: T) -> Game {
        Game {
            word: word.into(),
            guesses: BTreeSet::new(),
            strike_count: 0,
        }
    }

    pub fn check(&self) -> GameStateModel {
        GameStateModel {
            success: None,
            outcome: self.outcome(),
            state: self.build_state_string(),
            strike_count: self.strike_count,
            correct_word: None,
        }
    }

    pub fn guess(&mut self, guess: &str) -> GameStateModel {
        let guess = guess.chars().nth(0).unwrap_or(0 as char);
        let success = self.word.chars().any(|c| guess == c);

        self.guesses.insert(guess);
        self.strike_count += if success {
            0
        } else {
            1
        };

        let outcome = self.outcome();

        GameStateModel {
            success: Some(success),
            state: self.build_state_string(),
            strike_count: self.strike_count,
            outcome: outcome,
            correct_word: match outcome {
                Outcome::Lost => Some(self.word.to_owned()),
                _ => None,
            }
        }
    }

    fn build_state_string(&self) -> String {
        self.word.chars().map(|c| if self.guesses.contains(&c) {
            c
        } else {
            '_'
        }).collect()
    }

    pub fn outcome(&self) -> Outcome {
        if self.strike_count > 5 {
            return Outcome::Lost
        }

        if self.word == self.build_state_string() {
            return Outcome::Won
        }

        Outcome::InProgress
    }
}

pub struct GameStore;

impl Key for GameStore {
    type Value = HashMap<String, Game>;
}