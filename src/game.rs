use std::collections::{HashMap, BTreeSet};
use iron::typemap::Key;
use model::GameStateModel;
use serde::ser::{Serialize, Serializer};

#[derive(Eq, PartialEq)]
pub enum Outcome {
    InProgress,
    Lost,
    Won,
}

impl AsRef<str> for Outcome {
    fn as_ref(&self) -> &str {
        match *self {
            Outcome::InProgress => "InProgress",
            Outcome::Lost => "Lost",
            Outcome::Won => "Won",
        }
    }
}

impl Serialize for Outcome {
    fn serialize<S: Serializer>(&self, s: &mut S) -> Result<(), S::Error> {
        s.serialize_str(self.as_ref())
    }
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
        match guess.chars().nth(0) {
            None => GameStateModel {
                success: Some(false),
                state: self.build_state_string(),
                strike_count: {
                    self.strike_count += 1;
                    self.strike_count
                },
                outcome: self.outcome(),
                correct_word: if self.outcome() == Outcome::Lost {
                    Some(self.word.to_owned())
                } else {
                    None
                }
            },
            Some(guess) => {
                let success = self.word.chars().any(|c| c == guess);
                self.guesses.insert(guess);
                GameStateModel {
                    success: Some(success),
                    state: self.build_state_string(),
                    strike_count: {
                        if !success {
                            self.strike_count += 1;
                        }
                        self.strike_count
                    },
                    outcome: self.outcome(),
                    correct_word: if self.outcome() == Outcome::Lost {
                        Some(self.word.to_owned())
                    } else {
                        None
                    }
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