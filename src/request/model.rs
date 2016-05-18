use hangman_data::model::{CreateGame, CreateToken, Difficulty};

#[derive(Deserialize)]
pub struct CreateTokenRequest {
    token: String,
}

impl<'a> CreateToken for &'a CreateTokenRequest {
    fn token(&self) -> &str {
        &self.token
    }
}

#[derive(Deserialize)]
pub struct TokenRecordRequest {
    token: String,
}

impl TokenRecordRequest {
    pub fn token(&self) -> &str {
        &self.token
    }
}

#[derive(Deserialize)]
pub struct CreateGameRequest {
    token: String,
    difficulty: Difficulty,
}

impl<'a> CreateGame for &'a CreateGameRequest {
    fn 
}