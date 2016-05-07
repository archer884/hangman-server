use game::Outcome;

#[derive(Serialize)]
pub struct CheckModel {
    #[serde(rename = "State")]
    pub state: String,
    
    #[serde(rename = "Outcome")]
    pub outcome: Outcome,
    
    #[serde(rename = "Guesses")]
    #[serde(skip_serializing_if="Vec::is_empty")]
    pub guesses: Vec<char>,
    
    #[serde(rename = "StrikeCount")]
    pub strike_count: i32,
}

#[derive(Serialize)]
pub struct GuessModel {
    #[serde(rename = "Success")]
    pub success: bool,
    
    #[serde(rename = "Outcome")]
    pub outcome: Outcome,
    
    #[serde(rename = "State")]
    pub state: String,
    
    #[serde(rename = "Guesses")]
    #[serde(skip_serializing_if="Vec::is_empty")]
    pub guesses: Vec<char>,
    
    #[serde(rename = "StrikeCount")]
    pub strike_count: i32,
}