use hangman_data::service::PgConnectionService;
use iron::typemap::Key;

pub struct Db;

impl Key for Db {
    type Value = PgConnectionService;
}