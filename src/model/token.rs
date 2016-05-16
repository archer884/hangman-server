#[derive(Deserialize)]
pub struct CreateTokenRequest {
    token: String,
}

#[derive(Deserialize)]
pub struct TokenRecordRequest {
    token: String,
}