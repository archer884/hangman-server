use serde::ser::{Serialize, Serializer};

#[derive(Copy, Clone, Eq, PartialEq)]
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