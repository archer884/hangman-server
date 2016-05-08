#![feature(box_syntax, custom_derive, plugin, question_mark)]
#![plugin(serde_macros)]

#[macro_use] extern crate router;
extern crate iron;
extern crate persistent;
extern crate rand;
extern crate serde;
extern crate serde_json;
extern crate stopwatch;
extern crate strsim;

mod error;
mod game;
mod handler;
mod model;
mod outcome;
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
    use stopwatch::Stopwatch;

    println!("Reading word list");

    let time = Stopwatch::start_new();
    let word_list = match std::env::args().nth(1).and_then(|path| File::open(&path).ok()) {
        None => vec![],
        Some(file) => BufReader::new(file).lines()
            .filter_map(|line| line.map(|line| line.trim().to_ascii_lowercase()).ok())
            .filter(|word| words::validate_word(word))
            .collect()
    };

    let mut ranked_word_list: Vec<_> = word_list.iter().map(|word| (word, words::rank_word(word, &word_list))).collect();
    ranked_word_list.sort_by(|&(_, a), &(_, b)| b.cmp(&a));

    println!("Word list loaded in {}ms", time.elapsed_ms());
    ranked_word_list.iter().take(word_list.len() / 3).map(|&(word, _)| word.to_owned()).collect()
}