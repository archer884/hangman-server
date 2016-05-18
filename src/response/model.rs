use hangman_data::model::Record;

#[derive(Serialize)]
pub struct GenericResponse {
    success: bool,
    message: String,
}

impl GenericResponse {
    pub fn success<T: Into<String>>(message: T) -> GenericResponse {
        GenericResponse {
            success: true,
            message: message.into(),
        }
    }

    pub fn failure<T: Into<String>>(message: T) -> GenericResponse {
        GenericResponse {
            success: false,
            message: message.into(),
        }
    }
}

#[derive(Serialize)]
pub struct TokenRecordResponse {
    token: String,
    wins: i64,
    losses: i64,
}

impl TokenRecordResponse {
    pub fn from_record(record: &Record) -> TokenRecordResponse {
        TokenRecordResponse {
            token: record.token.to_owned(),
            wins: record.wins,
            losses: record.losses,
        }
    }
}

#[derive(Serialize)]
pub struct CreateGameResponse {
    token: String,
    id: i64,
}