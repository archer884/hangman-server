use clap::ArgMatches;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Difficulty {
    Hard,
    Normal,
    Easy,
}

pub struct Options {
    path: Option<String>,
    difficulty: Difficulty,
}

impl Options {
    pub fn read() -> Options {
        let matches = get_matches();
        Options {
            path: matches.value_of("path").map(|path| path.to_owned()),
            difficulty: if matches.is_present("hard") {
                Difficulty::Hard
            } else if matches.is_present("easy") {
                Difficulty::Easy
            } else {
                Difficulty::Normal
            }
        }
    }

    pub fn path(&self) -> Option<&String> {
        self.path.as_ref()
    }

    pub fn difficulty(&self) -> Difficulty {
        self.difficulty
    }
}

fn get_matches<'a>() -> ArgMatches<'a> {
    clap_app!(myapp =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: "A RESTful-ish Hangman service")
        (@arg path: "Dictionary path")
        (@group difficulty =>
            (@arg hard: -h --hard "Sets hard mode")
            (@arg normal: -n --normal "Sets normal mode")
            (@arg easy: -e --easy "Sets easy mode")
        )
    ).get_matches()
}