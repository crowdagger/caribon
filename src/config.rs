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

use std::env;
use std::process::exit;
use std::fs::File;
use std::error::Error;
use std::io;
use std::io::Read;
use std::io::Write;
use caribon::Parser;

static ARG_LANG:&'static str = "--language=";
static ARG_THRESHOLD:&'static str = "--threshold=";
static ARG_MAX_DISTANCE:&'static str = "--max-distance=";
static ARG_GLOBAL_THRESHOLD:&'static str = "--global-threshold=";
static ARG_HTML:&'static str = "--html=";
static ARG_IGNORE_PROPER:&'static str = "--ignore-proper=";
static ARG_USAGE:&'static str = "--help";
static ARG_INPUT:&'static str = "--input=";
static ARG_OUTPUT:&'static str = "--output=";
static ARG_VERSION:&'static str = "--version";
static ARG_LIST_LANGUAGES:&'static str = "--list-languages";
static ARG_IGNORE:&'static str = "--ignore=";

pub fn list_languages() {
    println!("Supported languages:");
    for l in Parser::list_languages() {
        println!("- '{}'", l);
    }
}

fn version() {
    println!("{}", env!("CARGO_PKG_VERSION"));
}

fn usage() {
    println!("
Caribon, version {} by Élisabeth Henry <liz.henry@ouvaton.org>

Detects the repetitions in a text and renders a HTML document highlighting them

Options:
{}: displays this message
{}: displays program version
{}: lists the implemented languages
{}[language]: sets the language of the text (default: french)
{}[filename]: sets input file (default: stdin)
{}[filename]: sets output file (default: stdout)
{}[string]: a string containing custom ignored words, separated by spaces or comma
    (default: use a builtin list that depends on the language)
{}[value]: sets max distance (used by local and leak algorithm) (default: 50)
{}[value]: sets threshold value for underlining local repetitions (default: 1.9)
{}[value]: sets threshold value for underlining global repetitions (default: 0.01)
{}[true|false]: enables/disable HTML input (default: true)
{}[true|false]: if true, try to detect proper nouns and don't count them (default: false)",
             env!("CARGO_PKG_VERSION"),
             ARG_USAGE,
             ARG_VERSION,
             ARG_LIST_LANGUAGES,
             ARG_LANG,
             ARG_INPUT,
             ARG_OUTPUT,
             ARG_IGNORE,
             ARG_MAX_DISTANCE,
             ARG_THRESHOLD,
             ARG_GLOBAL_THRESHOLD,
             ARG_HTML,
             ARG_IGNORE_PROPER);
}

pub struct Config {
    pub lang: String,
    pub threshold: f32,
    pub global_threshold: f32,
    pub max_distance: u32,
    pub html: bool,
    pub ignore_proper: bool,
    pub input: Box<Read>,
    pub output: Box<Write>,
    pub ignored: String,
}

impl Config {
    /// New default config
    pub fn new() -> Config {
        Config {
            lang: "french".to_string(),
            threshold:1.9,
            global_threshold: 0.01,
            max_distance:50,
            html:true,
            ignore_proper:false,
            input: Box::new(io::stdin()),
            output: Box::new(io::stdout()),
            ignored: String::new()
        }
    }

    /// New config from program args
    pub fn new_from_args() -> Config {
        let mut config = Config::new();
        let mut iter = env::args();
        iter.next();
        for argument in iter {
            config.parse_arg(&argument);
        }
        config
    }
    
    /// Parse a single argument
    pub fn parse_arg(&mut self, arg:&str) {
        if arg.starts_with(ARG_OUTPUT) {
            let option = &arg[ARG_OUTPUT.len()..];
            let result = File::create(option);
            match result {
                Ok(f) => self.output = Box::new(f),
                Err(e) => {
                    println!("Error opening file {}: {}", option, e.description());
                    exit(0);
                }
            }
        } else if arg.starts_with(ARG_INPUT) {
            let option = &arg[ARG_INPUT.len()..];
            let result = File::open(option);
            match result {
                Ok(f) => self.input = Box::new(f),
                Err(e) => {
                    println!("Error opening file {}: {}", option, e.description());
                    exit(0);
                }
            }
        } else if arg.starts_with(ARG_LANG) {
            let option = &arg[ARG_LANG.len()..];
            self.lang = option.to_string();
        } else if arg.starts_with(ARG_THRESHOLD) {
            let option = &arg[ARG_THRESHOLD.len()..];
            self.threshold = match option.parse() {
                Ok(x) => x,
                Err(_) => panic!("Error passing argument to threshold: {}", option),
            }
        } else if arg.starts_with(ARG_GLOBAL_THRESHOLD) {
            let option = &arg[ARG_GLOBAL_THRESHOLD.len()..];
            self.global_threshold = match option.parse() {
                Ok(x) => x,
                Err(_) => panic!("Error passing argument to threshold: {}", option),
            }
        } else if arg.starts_with(ARG_MAX_DISTANCE) {
            let option = &arg[ARG_MAX_DISTANCE.len()..];
            self.max_distance = match option.parse() {
                Ok(x) => x,
                Err(_) => panic!("Error passing argument to max_distance: {}", option),
            }
        } else if arg.starts_with(ARG_HTML) {
            let option = &arg[ARG_HTML.len()..];
            match option {
                "true" => self.html = true,
                "false" => self.html = false,
                _ => panic!("Wrong argument to html: expected 'true' or 'false', received: {}", option)
            }
        } else if arg.starts_with(ARG_IGNORE_PROPER) {
            let option = &arg[ARG_IGNORE_PROPER.len()..];
            match option {
                "true" => self.ignore_proper = true,
                "false" => self.ignore_proper = false,
                _ => panic!("Wrong argument to ignore_proper: expected 'true' or 'false', received: {}", option)
            }
        } else if arg.starts_with(ARG_IGNORE) {
            let option = &arg[ARG_IGNORE.len()..];
            self.ignored = option.to_string();
        } else if arg == ARG_USAGE {
            usage();
            exit(0);
        } else if arg == ARG_VERSION {
            version();
            exit(0);
        } else if arg == ARG_LIST_LANGUAGES {
            list_languages();
            exit(0);
        } else {
            println!("Unrecognized argument: {}. See {} for help", arg, ARG_USAGE);
            exit(0);
        }
    }
}
