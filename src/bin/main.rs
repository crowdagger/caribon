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

extern crate caribon;
mod config;
use config::Config;
use caribon::Parser;

use std::error::Error;
use std::io;
use std::io::Read;
use std::io::Write;
use std::io::BufRead;
use std::collections::HashMap;
use std::fs::File;

fn print_stats(h: &HashMap<String, f32>, n_words: u32) {
    let different_words = h.len();
    println!("Number of words: {}", n_words);
    println!("Number of different words: {}", different_words);
}

fn try_parse() -> Result<(), Box<Error>> {
    let mut config = Config::new_from_args();
    let mut parser = try!(Parser::new(&config.lang));

    parser = parser.with_html(&config.input_format == "html")
                   .with_fuzzy(config.fuzzy)
                   .with_ignore_proper(config.ignore_proper)
                   .with_max_distance(config.max_distance);

    if !config.ignored.is_empty() {
        parser = parser.with_ignored(&config.ignored);
    }
    if !config.add_ignored.is_empty() {
        parser = parser.with_more_ignored(&config.add_ignored);
    }
    if !config.ispell {
        let mut s = String::new();
        try!(config.input.read_to_string(&mut s));
        
        let mut ast = try!(parser.tokenize(&s));
        if config.print_stats {
        let (h, count) = parser.words_stats(&ast);
            print_stats(&h, count);
        }
        parser.detect_local(&mut ast, config.threshold);
        if let Some(threshold) = config.global_threshold {
            parser.detect_global(&mut ast, threshold);
        }
        let output = match &*config.output_format {
            "html" => parser.ast_to_html(&mut ast, true),
            "terminal" => parser.ast_to_terminal(&ast),
            "markdown" => parser.ast_to_markdown(&ast),
            _ => return Err(Box::new(caribon::Error::new("Wrong output format: must be 'html, \
                                                          'terminal', or 'markdown'"))),
        };
        try!(config.output.write(&output.bytes().collect::<Vec<u8>>()));
        Ok(())
    } else {
        config::disguise_as_ispell();

        let stdin = io::stdin();
        let mut i = 0;
        for line in stdin.lock().lines() {
            i += 1;
            let mut line = try!(line);
            let first = if let Some(c) = line.chars().next() {
                c
            } else {
                continue;
            };
            let mut offset = 0;
            match first {
                '!' => config.ispell_list = true, //terse mode
                '%' => config.ispell_list = false, //exit terse mode
                '*' | '@' | '#' | '~' | '+' | '-' => continue,
                '^' => {
                    offset = 1;
                    line = (&line[1..]).to_owned();
                },
                _ => (),
            }
            let mut ast = try!(parser.tokenize(&line));
            parser.detect_local(&mut ast, config.threshold);
            let res = parser.ast_to_ispell(&ast, config.ispell_list, offset);
            print!("{}", res);
            if !config.ispell_list {
                println!("");
            }
            // let mut f = File::create(&format!("/tmp/caribon{}.log", i)).unwrap();
            // f.write_all(&format!("input: {}\n***\noutput: {}\n", &line, res).as_bytes()).unwrap();
        }
        
        Ok(())
    }
}


fn main() {
    match try_parse() {
        Ok(_) => {}
        Err(e) => println!("{}", e.description()),
    }
}
