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
static ARG_ALGO:&'static str = "--algo=";
static ARG_LEAK:&'static str = "--leak=";
static ARG_THRESHOLD:&'static str = "--threshold=";
static ARG_MAX_DISTANCE:&'static str = "--max_distance=";
static ARG_HTML:&'static str = "--html=";
static ARG_IGNORE_PROPER:&'static str = "--ignore_proper=";
static ARG_GLOBAL_COUNT:&'static str = "--global_count=";
static ARG_USAGE:&'static str = "--help";
static ARG_INPUT:&'static str = "--input=";
static ARG_OUTPUT:&'static str = "--output=";
static ARG_VERSION:&'static str = "--version";
static ARG_LIST_LANGUAGES:&'static str = "--list_languages";
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

Reads a file on stdint and outputs an HTML file showing the repetitions

Options:
{}: displays this message
{}: displays program version
{}: lists the implemented languages
{}[language]: sets the language of the text (default: french)
{}[filename]: sets input file (default: stdin)
{}[filename]: sets output file (default: stdout)
{}[global|local|leak]: sets the detection algoritm (default: local)
{}[value]: sets leak value (only used by leak algorithm) (default: 0.95)
{}[value]: sets max distance (only used by local algorithm) (default: 50)
{}[relative|absolute]: sets repetitions count as absolute or relative ratio of words
                       (only used by global algorithm) (default: absolute)
{}[value]: sets threshold value for underlining repetitions (default: 1.9)
{}[true|false]: enables/disable HTML input (default: true)
{}[true|false]: if true, try to detect proper nouns and don't count them (default: false)",
             env!("CARGO_PKG_VERSION"),
             ARG_USAGE,
             ARG_VERSION,
             ARG_LIST_LANGUAGES,
             ARG_LANG,
             ARG_INPUT,
             ARG_OUTPUT,
             ARG_ALGO,
             ARG_LEAK,
             ARG_MAX_DISTANCE,
             ARG_GLOBAL_COUNT,
             ARG_THRESHOLD,
             ARG_HTML,
             ARG_IGNORE_PROPER);
}

pub enum Algorithm {
    Local,
    Global,
    Leak
}

pub struct Config {
    pub lang: String,
    pub algo: Algorithm,
    pub leak: f32,
    pub threshold: f32,
    pub max_distance: u32,
    pub html: bool,
    pub ignore_proper: bool,
    pub is_relative: bool,
    pub input: Box<Read>,
    pub output: Box<Write>,
    pub ignored: String,
}

impl Config {
    /// New default config
    pub fn new() -> Config {
        Config {
            lang: "french".to_string(),
            algo: Algorithm::Local,
            leak:0.95,
            threshold:1.9,
            max_distance:50,
            html:true,
            ignore_proper:false,
            is_relative:false,
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
        } else if arg.starts_with(ARG_ALGO) {
            let option = &arg[ARG_ALGO.len()..];
            match option {
                "leak" => self.algo = Algorithm::Leak,
                "local" => self.algo = Algorithm::Local,
                "global" => self.algo = Algorithm::Global,
                _ => {
                    println!("Unrecognized algorithm: {}", option);
                    exit(0);
                }
            }
        } else if arg.starts_with(ARG_LANG) {
            let option = &arg[ARG_LANG.len()..];
            self.lang = option.to_string();
        } else if arg.starts_with(ARG_LEAK) {
            let option = &arg[ARG_LEAK.len()..];
            self.leak = match option.parse() {
                Ok(x) => x,
                Err(_) => panic!("Error passing argument to leak: {}", option),
            }
        } else if arg.starts_with(ARG_THRESHOLD) {
            let option = &arg[ARG_THRESHOLD.len()..];
            self.threshold = match option.parse() {
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
        } else if arg.starts_with(ARG_GLOBAL_COUNT) {
            let option = &arg[ARG_GLOBAL_COUNT.len()..];
            match option {
                "relative" => self.is_relative = true,
                "absolute" => self.is_relative = false,
                _ => panic!("Wrong argument to global_count: expected 'relative' or 'absolute', received: {}", option)
            }
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
