use game::{Game, GameStore, Outcome};
use iron::prelude::*;
use iron::status;
use persistent::{Read, Write};
use rand::{Rng, thread_rng};
use request::Data;
use words::WordList;
use serde::ser::Serialize;
use serde_json::to_string as serialize;

pub fn check(req: &mut Request) -> IronResult<Response> {
    let token = req.token()?.to_owned();
    let mutex = req.get::<Write<GameStore>>().expect("gamestore not found");

    let mut games = mutex.lock().expect("unable to lock mutex");
    let game = games.entry(token).or_insert_with(|| Game::new(select_word(req)));

    if game.outcome() != Outcome::InProgress {
        *game = Game::new(select_word(req));
    }

    Ok(create_response(&game.check()))
}

pub fn guess(req: &mut Request) -> IronResult<Response> {
    let token = req.token()?.to_owned();
    let guess = req.guess()?;
    let mutex = req.get::<Write<GameStore>>().expect("gamestore not found");

    let mut games = mutex.lock().expect("unable to lock mutex");
    let mut game = games.entry(token).or_insert_with(|| Game::new(select_word(req)));

    if game.outcome() != Outcome::InProgress {
        *game = Game::new(select_word(req));
    }

    Ok(create_response(&game.guess(&guess)))
}

fn create_response<T: Serialize>(model: &T) -> Response {
    use iron::headers::ContentType;

    let mut response = Response::with((
        status::Ok,
        serialize(model).unwrap(),
    ));
    response.headers.set(ContentType::json());
    response
}

fn select_word(req: &mut Request) -> String {
    let word_list = req.get::<Read<WordList>>().expect("wordlist not found");
    thread_rng().choose(&word_list).map_or("apple".to_owned(), |word| word.to_owned())
}