#![feature(box_syntax, custom_derive, plugin, question_mark)]
#![plugin(serde_macros)]

#[macro_use] extern crate clap;
#[macro_use] extern crate router;
extern crate hangman_data;
extern crate iron;
extern crate persistent;
extern crate rand;
extern crate serde;
extern crate serde_json;
extern crate stopwatch;

mod data;
mod error;
mod handler;
mod options;
mod request;
mod words;

fn main() {
    use data::Db;
    use hangman_data::service::PgConnectionService;
    use iron::prelude::*;
    use persistent::{Read, Write};
    use words::WordList;

    let mut chain = Chain::new(router! {
        post "/api/v1/tokens/create" => handler::create_token,
        get "/api/v1/tokens/:token" => handler::record,
        post "/api/v1/games/create" => handler::create_game,
        post "/api/v1/games/:id" => handler::guess,
        get "/api/v1/games/:id" => handler::game_status,
        get "/api/v1/games/:token/:page" => handler::page_games, 
    });

    chain.link(Write::<Db>::both(PgConnectionService::new()));
    chain.link(Read::<WordList>::both(get_word_service()));

    println!("Serving requests");
    Iron::new(chain).http("localhost:1337").unwrap();
}

fn get_word_service() -> words::WordService {
    use std::fs::File;
    use std::io::BufReader;
    use options::Options;
    use stopwatch::Stopwatch;

    let options = Options::read();

    println!("Reading word list");

    let time = Stopwatch::start_new();
    let word_list = match options.path().and_then(|path| File::open(&path).ok()) {
        None => words::WordService::empty(),
        Some(file) => words::WordService::create(&mut BufReader::new(file)),
    };

    println!("Word list loaded in {}ms", time.elapsed_ms());
    word_list
}