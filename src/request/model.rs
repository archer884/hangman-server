use hangman_data::model::CreateToken;

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