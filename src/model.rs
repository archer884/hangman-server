use game::Outcome;

#[derive(Serialize)]
pub struct GameStateModel {
    #[serde(rename = "Success")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub success: Option<bool>,

    #[serde(rename = "State")]
    pub state: String,

    #[serde(rename = "Outcome")]
    pub outcome: Outcome,

    #[serde(rename = "StrikeCount")]
    pub strike_count: i32,

    #[serde(rename = "CorrectWord")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub correct_word: Option<String>,
}