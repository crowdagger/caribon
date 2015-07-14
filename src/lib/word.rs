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

/// `Word` type: some inner representation used by `Parser`.
///
/// You probably should not use this type directly.
#[derive(Debug,Clone)]
pub enum Word {
    /// A String which is not part of the text (typically whitespace, HTML formatting, ...)
    Untracked(String),
    /// A word that is ignored, either because it is in `parser.ignored` or because it is
    /// a proper noun and `ignore_proper` has been set to `true`.
    Ignored(String),
    /// Tracked string, containing the string, the stemmed variant of the
    /// string, some value corresponding to the degree of repetitions and
    /// an option to a highlighting colour
    Tracked(String, String, f32, Option<&'static str>),
}

impl Word {
    /// Sets the stemmed value of a word.
    pub fn set_stemmed(&mut self, s:String) {
        match self {
            &mut Word::Tracked(_, ref mut stemmed, _, _) => {
                *stemmed = s;
            },
            _ => {}
        }
    }
    
    /// Sets the repetition value of a word.
    pub fn set_count(&mut self, x: f32)  {
        match self {
            &mut Word::Tracked(_, _, ref mut v, _) => {
                *v = x;
            },
            _ => {}
        }
    }
}

#[derive(Debug,Clone)]
/// The internal representation of the document.
///
/// Technically the name AST is not really well chosen, since it is not a tree, but mainly a vector of
/// `Word`s plus some additonal informations for HTML parsing, but the idea is the same.
pub struct Ast {
    /// Vector of `Word`s. The main data of the structure.
    pub words: Vec<Word>,
    /// Position of <head> tag, if any
    pub begin_head: Option<usize>,
    /// position of <body> tag, if any
    pub begin_body: Option<usize>,
    /// position of </body> tag, if any
    pub end_body: Option<usize>,
}

impl Ast {
    /// Creates a new, empty AST
    pub fn new() -> Ast {
        Ast {
            words: vec!(),
            begin_head: None,
            begin_body: None,
            end_body: None
        }
    }

    /// Sets begin_head to current last position of words
    ///
    /// This should be called *before* inserting the corresponding element.
    pub fn mark_begin_head(&mut self) {
        if self.begin_head.is_some() {
            return;
        }
        
        let i = self.words.len();
        self.begin_head = Some(i);
    }

    /// Sets begin_body to current last position of words
    ///
    /// This should be called *before* inserting the corresponding element.
    pub fn mark_begin_body(&mut self)  {
        if self.begin_body.is_some() {
            return;
        }
        
        let i = self.words.len();
        self.begin_body = Some(i);
    }
    
    /// Sets end_body to current last position of words
    ///
    /// This should be called *before* inserting the corresponding element.
    pub fn mark_end_body(&mut self) {
        let i = self.words.len();
        self.end_body = Some(i);
    }

    /// Get only the words contained between <body> and </body>
    ///
    /// If body_begin and body_end are both set (and the first one is before the second),
    /// returns a slice that contains only words in this part; else, returns all words.
    pub fn get_body(&self) -> &[Word] {
        if let Some(begin) = self.begin_body {
            if let Some(end) = self.end_body {
                if begin < end {
                    return &self.words[begin + 1..end];
                }
            }
        }
        return &self.words;
    }
}
