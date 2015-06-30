// This file is part of Caribon.
//
// Caribon is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 of the License, or
// (at your option) any later version.
//
// Caribon is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Caribon.  If not, see <http://www.gnu.org/licenses/>.

extern crate caribon;
mod config;
use config::Config;
use config::Algorithm;
use caribon::Parser;

use std::error::Error;
use std::io::Read;

fn try_parse () -> Result<(), Box<Error>> {
    let mut config = Config::new_from_args();
    let mut parser = try!(Parser::new(&config.lang));

    parser = parser.with_html(config.html)
        .with_ignore_proper(config.ignore_proper)
        .with_max_distance(config.max_distance);

    if !config.ignored.is_empty() {
        parser = parser.with_ignored(&config.ignored);
    }
        
    let mut s = String::new();
    try!(config.input.read_to_string(&mut s));
    
    let words = try!(parser.tokenize(&s));
    // let repetitions = match config.algo {
    //     Algorithm::Local => parser.detect_local(words),
    //     Algorithm::Global => parser.detect_global(words, config.is_relative),
    //     Algorithm::Leak => parser.detect_leak(words)
    // };
//    let repetitions = parser.detect_leak(words);
    //    let repetitions = parser.highlight(repetitions, 1.5, "red");
    let repetitions = parser.detect_local(words);
    let repetitions = parser.highlight(repetitions, config.threshold, "red");
    let repetitions = parser.detect_global(repetitions, true);
    let repetitions = parser.highlight(repetitions, 0.01, "green");


    let html = parser.highlight_to_html(&repetitions, true);


    
//    let html = parser.words_to_html(&repetitions, config.threshold, true);
    try!(config.output.write(&html.bytes().collect::<Vec<u8>>()));
    Ok(())
}
    

fn main() {
    match try_parse () {
        Ok(_) => {},
        Err(e) => println!("{}", e.description())
    }
}
