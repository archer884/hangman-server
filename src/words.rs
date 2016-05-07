use iron::typemap::Key;

pub struct WordList;

impl Key for WordList {
    type Value = Vec<String>;
}