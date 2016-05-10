#![feature(box_syntax, custom_derive, plugin, question_mark)]
#![plugin(serde_macros)]

#[macro_use] extern crate clap;
#[macro_use] extern crate router;
extern crate iron;
extern crate persistent;
extern crate rand;
extern crate serde;
extern crate serde_json;
extern crate stopwatch;

mod error;
mod game;
mod handler;
mod model;
mod options;
mod outcome;
mod ranking;
mod request;
mod words;

fn main() {
    use std::collections::HashMap;
    use game::GameStore;
    use iron::prelude::*;
    use persistent::{Read, Write};
    use words::WordList;

    let mut chain = Chain::new(router! {
        get "/:token" => handler::check,
        post "/:token/:letter" => handler::guess,
    });

    chain.link(Write::<GameStore>::both(HashMap::new()));
    chain.link(Read::<WordList>::both(word_list()));

    println!("Serving requests");
    Iron::new(chain).http("localhost:1337").unwrap();
}

fn word_list() -> Vec<String> {
    use std::ascii::AsciiExt;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use options::Options;
    use stopwatch::Stopwatch;

    let options = Options::read();

    println!("Reading word list");

    let time = Stopwatch::start_new();
    let word_list = match options.path().and_then(|path| File::open(&path).ok()) {
        None => vec![],
        Some(file) => BufReader::new(file).lines()
            .filter_map(|line| match line {
                Ok(ref line) if line.is_ascii() => Some(line.trim().to_ascii_lowercase()),
                _ => None,
            })
            .filter(|word| words::validate_word(word))
            .collect()
    };

    let word_list = words::select_by_difficulty(options.difficulty(), word_list);

    println!("Word list loaded in {}ms", time.elapsed_ms());
    word_list
}