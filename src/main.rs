extern crate caribon;
mod config;
use config::Config;
use config::Algorithm;
use caribon::Parser;

use std::io;
use std::io::Read;

fn main() {
    let config = Config::new_from_args();
    if config.debug {
        println!("config:{:?}", config);
    }
    let result = Parser::new(&config.lang);

    let parser = match result {
        None => {
            println!("Language '{}' is not supported.", &config.lang);
            config::list_languages();
            return;
        },
        Some(x) => x
    };
    let parser = parser.with_html(config.html)
        .with_ignore_proper(config.ignore_proper)
        .with_max_distance(config.max_distance)
        .with_leak(config.leak);
        
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    
    let words = parser.tokenize(&s);
    let repetitions = match config.algo {
        Algorithm::Local => parser.detect_local(words),
        Algorithm::Global => parser.detect_global(words, config.is_relative),
        Algorithm::Leak => parser.detect_leak(words)
    };
    if config.debug {
        println!("{:?}", repetitions);
    } else {
        let html = caribon::words_to_html(&repetitions, config.threshold);
        println!("{}", html);
    }
}
