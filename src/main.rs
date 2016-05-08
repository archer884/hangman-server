#![feature(box_syntax, custom_derive, plugin, question_mark)]
#![plugin(serde_macros)]

#[macro_use] extern crate router;
extern crate iron;
extern crate persistent;
extern crate rand;
extern crate serde;
extern crate serde_json;

mod error;
mod game;
mod handler;
mod model;
mod request;
mod words;

use std::ascii::AsciiExt;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use game::GameStore;
use iron::prelude::*;
use persistent::{Read, Write};
use words::WordList;

fn main() {
    let mut chain = Chain::new(router! {
        get "/:token" => handler::check,
        post "/:token/:letter" => handler::guess,
    });
    
    chain.link(Write::<GameStore>::both(HashMap::new()));
    chain.link(Read::<WordList>::both(word_list()));
    
    Iron::new(chain).http("localhost:1337").unwrap();
}

fn word_list() -> Vec<String> {
    match std::env::args().nth(1).and_then(|path| File::open(&path).ok()) {
        None => vec![],
        Some(file) => BufReader::new(file).lines()
            .filter_map(|line| line.map(|line| line.trim().to_ascii_lowercase()).ok())
            .filter(|word| words::validate_word(word))
            .collect()
    }
}