use data::Db;
use error::ApplicationError;
use hangman_data::service::{ConnectionService, GameService, TokenService};
use iron::prelude::*;
use iron::status;
use iron::status::Status;
use persistent::Write;
use request::RequestData;
use request::model::*;
use response::model::*;
use serde::ser::Serialize;
use serde_json::to_string as serialize;

pub fn create_token(req: &mut Request) -> IronResult<Response> {
    let request = req.body::<CreateTokenRequest>()?;
    let mutex = req.get::<Write<Db>>().expect("Db not found");
    let mut db = mutex.lock().expect("unable to lock mutex");

    match db.tokens().create(&request) {
        Err(_) => build_response(
            &GenericResponse::failure("token not added"),
            status::Conflict,
        ),
        Ok(_) => build_response(
            &GenericResponse::success("token added"),
            status::Ok,
        ),
    }
}

pub fn record(req: &mut Request) -> IronResult<Response> {
    let request = req.body::<TokenRecordRequest>()?;
    let mutex = req.get::<Write<Db>>().expect("Db not found");
    let mut db = mutex.lock().expect("unable to lock mutex");
    
    match db.tokens().record(request.token()) {
        Err(e) => Err(ApplicationError::Db(box e).into()),
        Ok(ref record) => build_response(&TokenRecordResponse::from_record(record), status::Ok),
    }
}

pub fn create_game(req: &mut Request) -> IronResult<Response> {
    unimplemented!()
}

pub fn guess(req: &mut Request) -> IronResult<Response> {
    unimplemented!()
}

pub fn game_status(req: &mut Request) -> IronResult<Response> {
    unimplemented!()
}

pub fn page_games(req: &mut Request) -> IronResult<Response> {
    unimplemented!()
}

fn build_response<T: Serialize>(model: &T, status: Status) -> IronResult<Response> {
    use iron::headers::ContentType;

    let mut response = Response::with((status, serialize(model).unwrap()));
    response.headers.set(ContentType::json());
    Ok(response)
}

// pub fn check(req: &mut Request) -> IronResult<Response> {
//     let token = req.token()?.to_owned();
//     let mutex = req.get::<Write<GameStore>>().expect("gamestore not found");

//     let mut games = mutex.lock().expect("unable to lock mutex");
//     let game = games.entry(token).or_insert_with(|| Game::new(select_word(req)));

//     if game.outcome() != Outcome::InProgress {
//         *game = Game::new(select_word(req));
//     }

//     Ok(create_response(&game.check()))
// }

// pub fn guess(req: &mut Request) -> IronResult<Response> {
//     let token = req.token()?.to_owned();
//     let guess = req.guess()?;
//     let mutex = req.get::<Write<GameStore>>().expect("gamestore not found");

//     let mut games = mutex.lock().expect("unable to lock mutex");
//     let mut game = games.entry(token).or_insert_with(|| Game::new(select_word(req)));

//     if game.outcome() != Outcome::InProgress {
//         *game = Game::new(select_word(req));
//     }

//     Ok(create_response(&game.guess(&guess)))
// }

// fn create_response<T: Serialize>(model: &T) -> Response {
//     use iron::headers::ContentType;

//     let mut response = Response::with((
//         status::Ok,
//         serialize(model).unwrap(),
//     ));
//     response.headers.set(ContentType::json());
//     response
// }

// fn select_word(req: &mut Request) -> String {
//     use rand::{Rng, thread_rng};
//     use words::WordList;

//     let word_list = req.get::<Read<WordList>>().expect("wordlist not found");
//     word_list.choose().map_or("apple".to_owned(), |word| word.to_owned())
// }