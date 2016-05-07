use std::collections::{HashMap, BTreeSet};
use iron::typemap::Key;
use model::{CheckModel, GuessModel};

#[derive(Eq, PartialEq, Serialize)]
pub enum Outcome {
    InProgress,
    Lost,
    Won,
}

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
    
    pub fn check(&self) -> CheckModel {
        CheckModel {
            outcome: self.outcome(),
            state: self.build_state_string(),
            guesses: self.guesses_as_vec(),
            strike_count: self.strike_count,
        }
    }
    
    pub fn guess(&mut self, guess: &str) -> GuessModel {
        match guess.chars().nth(0) {
            None => GuessModel {
                success: false,
                state: self.build_state_string(),
                guesses: self.guesses_as_vec(),
                strike_count: {
                    self.strike_count += 1;
                    self.strike_count
                },
                outcome: self.outcome(),
            },
            Some(guess) => {
                let success = self.word.chars().any(|c| c == guess);
                self.guesses.insert(guess);
                GuessModel {
                    success: success,
                    state: self.build_state_string(),
                    guesses: self.guesses_as_vec(),
                    strike_count: {
                        if !success {
                            self.strike_count += 1;
                        }
                        self.strike_count
                    },
                    outcome: self.outcome(),                    
                }
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
    
    fn guesses_as_vec(&self) -> Vec<char> {
        self.guesses.iter().cloned().collect()
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