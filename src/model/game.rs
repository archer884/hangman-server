#[derive(Deserialize)]
pub struct CreateGameRequest {
    #[serde(rename = "Token")]
    token: String,
    
    #[serde(rename = "Difficulty")]
    difficulty: Difficulty,
}

#[derive(Serialize)]
pub struct CreateGameResponse<'a> {
    #[serde(rename = "Id")]
    id: i64,
    
    #[serde(rename = "Status")]
    status: &'a str,
}

#[derive(Deserialize)]
pub struct GameStatusRequest {
    #[serde(rename = "Token")]
    token: String,
}

#[derive(Deserialize)]
pub struct GameGuessRequest {
    #[serde(rename = "Token")]
    token: String,
    
    #[serde(rename = "Guess")]
    guess: String,
}

#[derive(Deserialize)]
pub struct GamePageRequest {
    #[serde(rename = "Token")]
    token: String,
    
    #[serde(rename = "Page")]
    page: i64,
}

#[derive(Serialize)]
pub struct GamePageResponse<'a> {
    #[serde(rename = "Games")]
    games: Vec<StatusResponse<'a>>
}

#[derive(Serialize)]
pub struct StatusResponse<'a> {
    #[serde(rename = "Id")]
    id: i64,
    
    #[serde(rename = "Status")]
    status: &'a str,
    
    #[serde(rename = "Strikes")]
    strikes: i32,
    
    #[serde(rename = "Outcome")]
    outcome: Outcome,
    
    #[serde(rename = "CorrectWord")]
    #[serde(skip_serializing_if="Option::is_none")]
    correct_word: Option<&'a str>,
}