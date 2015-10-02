// Copyright (C) 2015 Élisabeth HENRY.
//
// This file is part of Caribon.
//
// Caribon is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published
// by the Free Software Foundation, either version 2.1 of the License, or
// (at your option) any later version.
//
// Caribon is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with Caribon.  If not, see <http://www.gnu.org/licenses/>.

use std::env;
use std::process::exit;
use std::fs::File;
use std::error::Error;
use std::io;
use std::io::Read;
use std::io::Write;
use caribon::Parser;

const ARG_LANG:&'static str = "--language=";
const ARG_THRESHOLD:&'static str = "--threshold=";
const ARG_MAX_DISTANCE:&'static str = "--max-distance=";
const ARG_GLOBAL_THRESHOLD:&'static str = "--global-threshold=";
const ARG_INPUT_FORMAT:&'static str = "--input-format=";
const ARG_OUTPUT_FORMAT:&'static str = "--output-format=";
const ARG_IGNORE_PROPER:&'static str = "--ignore-proper=";
const ARG_USAGE:&'static str = "--help";
const ARG_INPUT:&'static str = "--input=";
const ARG_OUTPUT:&'static str = "--output=";
const ARG_VERSION:&'static str = "--version";
const ARG_LIST_LANGUAGES:&'static str = "--list-languages";
const ARG_IGNORE:&'static str = "--ignore=";
const ARG_ADD_IGNORED:&'static str = "--add-ignored=";
const ARG_FUZZY:&'static str = "--fuzzy=";
const ARG_STATS:&'static str = "--print-stats";

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

Detects the repetitions in a text and highlights them

Options:
  {}: displays this message
  {}: displays program version
  {}: lists the implemented languages
  {}: in addition to detecting repetition, displays some statistics on the input text
  {}[language]: sets the language of the text (default: french)
  {}[filename]: sets input file (default: stdin)
  {}[filename]: sets output file (default: stdout)
  {}[string]: sets ignored word to those contained in the string separated by spaces or comma
  \t(default: the builtin list that depends on the language)
  {}[string]: adds words contained in the string to the list of ignored words (default: none)
  {}[value]: sets max distance to be considered a repetition (in words) (default: 50)
  {}[value]: sets threshold value for underlining local repetitions (default: 1.9)
  {}[value|none]: activate global repetition detector and sets threshold value for underlining global repetitions
  \t(this threshold corresponds to the minimal ratio of words in the text, e.g. a threshold of 0.01 means
  \tthat a word must represent at least 1% of the total words in the text to be underlined) (default: not activated)
  {}[text|html]: sets input format (default: text, depends on input file extension)
  {}[terminal|html]|markdown]: sets output format (default: terminal, depends on output file extension)
  {}[true|false]: if true, try to detect proper nouns and don't count them (default: false)
  {}[value|none]: activate fuzzy string matching; value must be between 0.0 and 1.0 and corresponds to the maximal
  \t'difference' between two words until they are no more considered identical (e.g. 0.25 means that two words
  \t must have no more than 25% of difference) (default: not activated)",
             env!("CARGO_PKG_VERSION"),
             ARG_USAGE,
             ARG_VERSION,
             ARG_LIST_LANGUAGES,
             ARG_STATS,
             ARG_LANG,
             ARG_INPUT,
             ARG_OUTPUT,
             ARG_IGNORE,
             ARG_ADD_IGNORED,
             ARG_MAX_DISTANCE,
             ARG_THRESHOLD,
             ARG_GLOBAL_THRESHOLD,
             ARG_INPUT_FORMAT,
             ARG_OUTPUT_FORMAT,
             ARG_IGNORE_PROPER,
             ARG_FUZZY);
}

pub struct Config {
    pub lang: String,
    pub threshold: f32,
    pub global_threshold: Option<f32>,
    pub max_distance: u32,
    pub input_format: String,
    pub output_format: String,
    pub ignore_proper: bool,
    pub input: Box<Read>,
    pub input_filename: String,
    pub output: Box<Write>,
    pub output_filename: String,
    pub ignored: String,
    pub add_ignored: String,
    pub fuzzy: Option<f32>,
    pub print_stats: bool,
}

impl Config {
    /// New default config
    pub fn new() -> Config {
        Config {
            lang: "french".to_owned(),
            threshold:1.9,
            global_threshold: None,
            max_distance:50,
            input_format: String::new(),
            output_format: String::new(),
            ignore_proper:false,
            input: Box::new(io::stdin()),
            input_filename: String::new(),
            output: Box::new(io::stdout()),
            output_filename: String::new(),
            ignored: String::new(),
            add_ignored: String::new(),
            fuzzy: None,
            print_stats: false,
        }
    }

    /// New config from program args
    pub fn new_from_args() -> Config {
        let mut config = Config::new();
        let mut iter = env::args();
        iter.next();
        // Sets fields from args
        for argument in iter {
            config.parse_arg(&argument);
        }
        // Sets fields to default values if they have not been set
        if config.input_format.is_empty() {
            if config.input_filename.ends_with(".html") {
                config.input_format = "html".to_owned();
            } else {
                config.input_format = "text".to_owned();
            }
        }
        if config.output_format.is_empty() {
            if config.output_filename.ends_with(".html") {
                config.output_format = "html".to_owned();
            } else if config.output_filename.ends_with(".md") {
                config.output_format = "markdown".to_owned();
            } else {
                config.output_format = "terminal".to_owned();
            }
        }
        config
    }
    
    /// Parse a single argument
    pub fn parse_arg(&mut self, arg:&str) {
        if arg.starts_with(ARG_OUTPUT) {
            let option = &arg[ARG_OUTPUT.len()..];
            let result = File::create(option);
            match result {
                Ok(f) => {
                    self.output = Box::new(f);
                    self.output_filename = option.to_owned();
                },
                Err(e) => {
                    println!("Error opening file {}: {}", option, e.description());
                    exit(0);
                }
            }
        } else if arg.starts_with(ARG_INPUT) {
            let option = &arg[ARG_INPUT.len()..];
            let result = File::open(option);
            match result {
                Ok(f) => {
                    self.input = Box::new(f);
                    self.input_filename = option.to_owned();
                },
                Err(e) => {
                    println!("Error opening file {}: {}", option, e.description());
                    exit(0);
                }
            }
        } else if arg.starts_with(ARG_LANG) {
            let option = &arg[ARG_LANG.len()..];
            self.lang = option.to_owned();
        } else if arg.starts_with(ARG_THRESHOLD) {
            let option = &arg[ARG_THRESHOLD.len()..];
            self.threshold = match option.parse() {
                Ok(x) => x,
                Err(_) => {
                    println!("Error passing argument to threshold: {}", option);
                    exit(0);
                }
            }
        } else if arg.starts_with(ARG_FUZZY) {
            let option = &arg[ARG_FUZZY.len()..];
            self.fuzzy = if option == "none" {
                None
            } else {
                match option.parse() {
                    Ok(x) => Some(x),
                    Err(_) => {
                        println!("Error passing argument to fuzzy: {}", option);
                        exit(0);
                    }
                }
            }
        } else if arg.starts_with(ARG_GLOBAL_THRESHOLD) {
            let option = &arg[ARG_GLOBAL_THRESHOLD.len()..];
            self.global_threshold = if option == "none" {
                None
            } else {
                match option.parse() {
                    Ok(x) => Some(x),
                    Err(_) => {
                        println!("Error passing argument to global threshold: {}", option);
                        exit(0);
                    }
                }
            }
        } else if arg.starts_with(ARG_MAX_DISTANCE) {
            let option = &arg[ARG_MAX_DISTANCE.len()..];
            self.max_distance = match option.parse() {
                Ok(x) => x,
                Err(_) => {
                    println!("Error passing argument to max_distance: {}", option);
                    exit(0);
                }
            }
        } else if arg.starts_with(ARG_INPUT_FORMAT) {
            let option = &arg[ARG_INPUT_FORMAT.len()..];
            match option {
                "html" => self.input_format = option.to_owned(),
                "text" => self.input_format = option.to_owned(),
                _ => {
                    println!("Wrong argument to {}: expected 'html' or 'text', received: {}", ARG_INPUT_FORMAT, option);
                    exit(0);
                }
            }
        } else if arg.starts_with(ARG_OUTPUT_FORMAT) {
            let option = &arg[ARG_OUTPUT_FORMAT.len()..];
            match option {
                "html" => self.output_format = option.to_owned(),
                "terminal" => self.output_format = option.to_owned(),
                "markdown" => self.output_format = option.to_owned(),
                _ => {
                    println!("Wrong argument to {}: expected 'html', 'terminal' or 'markdown', received: {}", ARG_OUTPUT_FORMAT, option);
                    exit(0);
                }
            }
        } else if arg.starts_with(ARG_IGNORE_PROPER) {
            let option = &arg[ARG_IGNORE_PROPER.len()..];
            match option {
                "true" => self.ignore_proper = true,
                "false" => self.ignore_proper = false,
                _ => {
                    println!("Wrong argument to ignore_proper: expected 'true' or 'false', received: {}", option);
                    exit(0);
                }
            }
        } else if arg.starts_with(ARG_IGNORE) {
            let option = &arg[ARG_IGNORE.len()..];
            self.ignored = option.to_owned();
        } else if arg.starts_with(ARG_ADD_IGNORED) {
            let option = &arg[ARG_ADD_IGNORED.len()..];
            self.add_ignored = option.to_owned();
        } else if arg == ARG_USAGE {
            usage();
            exit(0);
        } else if arg == ARG_VERSION {
            version();
            exit(0);
        } else if arg == ARG_LIST_LANGUAGES {
            list_languages();
            exit(0);
        } else if arg == ARG_STATS {
            self.print_stats = true;
        } else {
            println!("Unrecognized argument: {}. See {} for help", arg, ARG_USAGE);
            exit(0);
        }
    }
}
