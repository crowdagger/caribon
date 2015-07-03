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
use caribon::Parser;

use std::error::Error;
use std::io::Read;

fn try_parse () -> Result<(), Box<Error>> {
    let mut config = Config::new_from_args();
    let mut parser = try!(Parser::new(&config.lang));

    parser = parser.with_html(&config.input_format == "html")
        .with_fuzzy(Some(0.25))
        .with_ignore_proper(config.ignore_proper)
        .with_max_distance(config.max_distance);

    if !config.ignored.is_empty() {
        parser = parser.with_ignored(&config.ignored);
    }
        
    let mut s = String::new();
    try!(config.input.read_to_string(&mut s));
    
    let words = try!(parser.tokenize(&s));
    let mut repetitions = parser.detect_local(words, config.threshold);
    repetitions = parser.detect_global(repetitions, config.global_threshold);
    let output = match &*config.output_format {
        "html" => parser.words_to_html(&repetitions, true),
        "terminal" => parser.words_to_terminal(&repetitions),
        "markdown" => parser.words_to_markdown(&repetitions),
        _ => return Err(Box::new(caribon::Error::new("Wrong output format: must be 'html, 'terminal', or 'markdown'")))
    };
    try!(config.output.write(&output.bytes().collect::<Vec<u8>>()));
    Ok(())
}
    

fn main() {
    match try_parse () {
        Ok(_) => {},
        Err(e) => println!("{}", e.description())
    }
}
