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
// You should have received ba copy of the GNU Lesser General Public License
// along with Caribon.  If not, see <http://www.gnu.org/licenses/>.

//! # Repetition detector library
//!
//! Detects the repetitions in an input file, using a stemming library in order to detect
//! words that are not technically identical but are, for all intents and purpose, essentially the
//! same, such as singular and plural (e.g. "cat" and "cats").
//!
//! Here's a short example (more details below):
//!
//! ```
//! use caribon::Parser;
//! let mut parser = Parser::new("english").unwrap(); //creates a new parser
//! let mut ast = parser.tokenize("Some text where you want to detect repetitions").unwrap();
//! parser.detect_local(&mut ast, 1.5);
//! parser.detect_global(&mut ast, 0.01); // wouldn't actually make much sense on a string so small
//! let html = parser.ast_to_html(&mut ast, true);
//! println!("{}", html);
//! ```
//!
//! You must first create a new `Parser`. Since the stemming algorithm is dependent on the language,
//! `Parser::new` takes a language as argument (or "no_stemmer" to disable stemming).
//!
//! `Parser::new` returns a `caribon::Result<Parser>`, which will contain `Ok(Parser)`
//! if the language is implemented, `Err(caribon::Error)` else:
//!
//! ```
//! use caribon::Parser;
//! let result = Parser::new("foo");
//! assert!(result.is_err()); // language "foo" is not implemented
//! ```
//!
//! Once you have a parser, you can then configure it with various options:
//!
//! ```
//! use caribon::Parser;
//! let mut parser = Parser::new("english").unwrap()
//! .with_html(true)
//! .with_ignore_proper(true)
//! .with_max_distance(20)
//! .with_ignored("some, words, to, ignore");
//! ```
//!
//! The next step is to read some string and convert it to the inner format (see the `Ast` structure).
//!
//! ```ignore
//! let mut ast = parser.tokenize(some_string).unwrap();
//! ```
//!
//! As `new`, this method can fail (typically on ill-formed HTML inputs), so it returns a `Result`.
//!
//! You then have a choice between multiple repetition detection algorithms. `detect_local` is probably
//! the one you want to use:
//!
//! ```ignore
//! parser.detect_local(&mut ast, 1.9);
//! ```
//!
//! There is also `detect_global`, which detects repetiion in the whole file:
//!
//! ```ignore
//! parser.detect_global(&mut ast, 0.01);
//! ```
//!
//! Each of these algorithms takes a theshold as argument, which is the minimal "amount of repetition" to
//! underline a word (in `detect_local`, this number is simply the number of occurrence of a word in a window of
//! `parser.max_distance` words, whereas, for `detect_global` it is the ratio of appeareance of a particular word).
//!
//! Once you have detected those repetitions, the final step is to print them.
//! `ast_to_html` does this. Besides a reference to an `Ast`, it takes one argument: a
//! boolean that tests whether the HTML code must be a standalone file or not (you will probably
//! want to set it to true).
//!
//! ```ignore
//! let html = parser.ast_to_html(&mut ast, true);
//! ```
//!
//! There are two other "outputting" methods: `ast_to_terminal` and `ast_to_markdown`:
//!
//! ```ignore
//! let output_terminal = parser.ast_to_terminal(&ast);
//! let output_markdown = parser.ast_to_markdown(&ast);
//! ```
//!
//! Both actually outputs texts; `ast_to_terminal` uses terminal color codes to highlight repetitions when the
//! string is displayed on a terminal, while `ast_to_markdown` uses markdown strong emphasis to highlight repetitions.
//!
//!

// Uncomment this if you use nightly and want to run benchmarks
//#![feature(test)]
//mod bench;

extern crate stemmer;
extern crate edit_distance;

mod error;
mod word;
mod parser;
mod display;


pub use error::Error;
pub use error::Result;
pub use word::Word;
pub use word::Ast;
pub use parser::Parser;
