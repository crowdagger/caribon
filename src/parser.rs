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

extern crate stemmer;
use self::stemmer::Stemmer;
use word::Word;
use std::collections::HashMap;

static START:&'static str = include_str!("html/start.html");
static END:&'static str = include_str!("html/end.html");

static IGNORED_FR:&'static str = "la le les pas ne nos des ils elles il elle se on nous vous leur leurs \
de et un une t s à d l je tu";
static IGNORED_EN:&'static str = "it s i of the a you we she he they them its their";

/// Parser which can load a string, detects repetition on it and outputs an HTML file
pub struct Parser {
    /// The stemmer 
    stemmer: Stemmer,
    /// List of ignored words: we don't want to count repetitions on them
    ignored: Vec<String>,
    /// Whether there is HTML in the input text
    html: bool,
    /// Ignores proper nouns
    ignore_proper: bool,
    /// Leak, only used for detect_leak
    leak: f32,
    /// Max distance to consider a repetition, only used for detect_local
    max_distance: u32
}

impl Parser {
    /// Returns a vector containing all languages implemented.
    ///
    /// These values are correct values to give to `Parser::new`.
    pub fn list_languages() -> Vec<&'static str> {
        Stemmer::list()
    }

    /// Returns a vector of ignored words from a string.
    ///
    /// # Arguments
    ///
    /// * `list` – A space or comma separated string, containing words that
    ///   should be ignored (i.e., don't count repetitions on them).
    pub fn get_ignored_from_string(list: &str) -> Vec<String> {
        list.split(|c: char| !c.is_alphabetic())
            .map(|s| s.to_string())
            .collect()
    }    
    
    /// Returns a vector containing the default ignored words for this language.
    pub fn get_ignored_from_lang(lang: &str) -> Vec<String> {
        match lang {
            "french" => Parser::get_ignored_from_string(IGNORED_FR),
            "english" => Parser::get_ignored_from_string(IGNORED_EN),
            _ => vec!()
        }
    }
    
    /// Returns `Some(Parser)` if language is `ok`, None else.
    ///
    /// # Arguments
    ///
    /// `lang` – The input text language. This will be used to create the
    ///          stemmer; it also determines what list of ignored words to use.
    ///
    /// # Example
    ///
    /// ```
    /// let result = caribon::Parser::new("english");
    /// assert!(result.is_some());
    /// ```
    pub fn new(lang: &str) -> Option<Parser> {
        let stemmer = Stemmer::new(lang);
        if stemmer.is_none() {
            return None;
        }
        let stemmer = stemmer.unwrap();
        let ignored = Parser::get_ignored_from_lang(lang);
        Some(Parser{stemmer: stemmer,
                    ignored: ignored,
                    html: true,
                    ignore_proper: false,
                    leak: 0.98,
                    max_distance: 50})
    }

    /// Sets max distance for repetitions (default 50).
    ///
    /// # Arguments
    ///
    /// `max_dist` – A number corresponding to a number of words. If two
    ///              occurences of a same word are separated by more than
    ///              this distance, it will not be counted as a repetition.
    pub fn with_max_distance(mut self, max_dist: u32) -> Parser {
        self.max_distance = max_dist;
        self
    }
   
    /// Sets HTML detection in input (default true).
    ///
    /// You should set it to `false` if a text is text-formatted, and to
    /// `true` if it contains HTML. 
    pub fn with_html(mut self, html: bool) -> Parser {
        self.html = html;
        self
    }

    /// Sets whether repetition detection should ignore proper nouns (default false).
    ///
    /// Basically, if set to `true`, words that start with a capital and are not at the beginning of
    /// a sentence won't be counted for repetitions. Currently, there are still counted if they are in the beginning of
    /// a sentence, but with most texts it won't be enough to highlight them as repetitions.
    pub fn with_ignore_proper(mut self, proper: bool) -> Parser {
        self.ignore_proper = proper;
        self
    }
    
    /// Sets the leak (default 0.98).
    ///
    /// Only used by `detect_leak` algorithm.
    pub fn with_leak(mut self, leak: f32) -> Parser {
        self.leak = leak;
        self
    }
        
    /// Sets the ignored list with a list of words contained in the argument string.
    ///
    /// # Arguments
    ///
    /// * `list` – A comma or whitespace separated list of words that should be ignored.
    pub fn with_ignored(mut self, list: &str) -> Parser {
        self.ignored = Parser::get_ignored_from_string(list);
        self
    }

    /// When we know it is the beginning of an escape character (e.g. &nbsp;)
    fn tokenize_escape<'b>(&self, c:&'b [char]) -> (&'b [char], Word) {
        let mut res = String::new();
        let mut chars:&[char] = c;

        loop {
            if chars.is_empty() {
                panic!("Error reading HTML: ill-formed escape code. Maybe this is not an HTML file?");
            }
            let c = chars[0];
            res.push(c);
            chars = &chars[1..];
            if c == ';' {
                return (chars, Word::Untracked(res));
            }
        }
    }
    
    fn tokenize_html<'b>(&self, c:&'b [char]) -> (&'b [char], Word) {
        let mut res = String::new();
        let mut chars:&[char] = c;

        let mut brackets = 1;
        res.push(chars[0]);
        chars = &chars[1..];

        loop {
            if chars.is_empty() {
                panic!("Error reading HTML: ill-formed HTML. Maybe this is not an HTML file?");
            }
            let c = chars[0];
            res.push(c);
            chars = &chars[1..];
            if c == '<' {
                brackets += 1;
            }
            if c == '>' {
                brackets -= 1;
                if brackets == 0 {
                    break;
                }
            }
        }

        (chars, Word::Untracked(res))
    }
    
    fn tokenize_whitespace<'b>(&self, c:&'b [char], is_begin: &mut bool) -> (&'b [char], Word) {
        let mut res = String::new();
        let mut chars:&[char] = c;

        loop {
            if chars.is_empty() {
                break;
            }
            let c = chars[0];
            if c == '<' || c == '&' || c.is_alphabetic() {
                break;
            }
            chars = &chars[1..];
            res.push(c);
            if c == '.' {
                *is_begin = true;
            }
        }

        (chars, Word::Untracked(res))
    }

    /// Return true if `s` is a proper noun, false else
    fn is_proper_noun(&self, s:&str, is_begin: bool) -> bool {
        if self.ignore_proper {
            if !is_begin {
                let o = s.chars().next();
                match o {
                    None => false,
                    Some(c) => c.is_uppercase()
                }
            }
            else {
                // Technically a proper noun could be at the beginning of a sentence :s
                false
            }
        } else {
            false
        }
    }

    fn tokenize_word<'b>(&self, c: &'b [char], is_begin:&mut bool) -> (&'b [char], Word) {
        let mut res = String::new();
        let mut chars:&[char] = c;
        
        loop {
            if chars.is_empty() {
                break;
            }
            let c = chars[0];
            if !c.is_alphabetic() {
                break;
            }
            res.push(c);
            chars = &chars[1..];
        }
        
        let lower_s:Vec<String> = res.chars()
            .map(|c| c.to_lowercase().collect::<String>())
            .collect();
        let lower_s = lower_s.concat();
        let word = if self.ignored.contains(&lower_s)
            || self.is_proper_noun(&res, *is_begin) {
            Word::Ignored(res)
        } else {
            Word::Tracked(res,
                          self.stemmer.stem(&lower_s),
                          0.0)
        };

        *is_begin = false;
        (chars, word)
    }


    /// Tokenize a string into a list of words. 
    ///
    /// This is the step that converts a string to some inner representation.
    ///
    /// # Arguments
    ///
    /// * `s` – The string to tokenize.
    pub fn tokenize(&self, s: &str) -> Vec<Word> {
        let v_chars:Vec<char> = s.chars().collect();
        let mut chars:&[char] = &v_chars;
        let mut res:Vec<Word> = vec!();
        let mut is_sentence_beginning = true;
        

        while !chars.is_empty() {
            let c = chars[0];
            if c.is_alphabetic() {
                let (c, word) = self.tokenize_word(chars, &mut is_sentence_beginning);
                chars = c;
                res.push(word);
            } else if self.html && c == '<' {
                let (c, word) = self.tokenize_html(chars);
                chars = c;
                res.push(word);
                is_sentence_beginning = false;
            } else if self.html && c == '&' {
                let (c, word) = self.tokenize_escape(chars);
                chars = c;
                res.push(word);
            } else {
                let (c, word) = self.tokenize_whitespace(chars, &mut is_sentence_beginning);
                chars = c;
                res.push(word);
            }
        }
     
        res
    }

    /// Detect the local repetitions using a leak value.
    ///
    /// Basically, each time a word occurs, increase value by 1.0
    /// and each time it does not, multiply by leak (default: 0.98).
    pub fn detect_leak(&self, mut vec:Vec<Word>) -> Vec<Word> {
        let mut h:HashMap<String, (u32, f32)> = HashMap::new();
        let mut pos = 0;

        for i in 0 .. vec.len() {
            match &mut vec[i] {
                &mut Word::Untracked(_) => {}
                &mut Word::Ignored(_) => {
                    pos += 1;
                },
                &mut Word::Tracked(_, ref stemmed, ref mut v) => {
                    pos += 1;
                    let x = match h.get(stemmed) {
                        None => 0.0,
                        Some(tmp) => {
                            let &(n, y) = tmp;
                            y * self.leak.powi((pos - n) as i32)
                        }
                    } + 1.0;
                    h.insert(stemmed.clone(), (pos, x));
                    *v = x;
                }
            };
        }
        vec
    }

    /// Detect the local number of repetitions.
    ///
    /// For each word, the repetition value is set to the total number of occurences of this word
    /// since there has been hat least `self.max_distance` between two occurences.
    ///
    /// It is the default algorithm, and probably the one you want to use.
    pub fn detect_local(&self, mut vec:Vec<Word>) -> Vec<Word> {
        let mut h:HashMap<String, (u32, Vec<usize>)> = HashMap::new(); 
        let mut pos = 1;

        for i in 0 .. vec.len() {
            let elem = match vec[i] {
                Word::Untracked(_) => None,
                Word::Ignored(_) => {
                    pos += 1;
                    None
                },
                Word::Tracked(_, ref stemmed, _) => {
                    pos += 1;
                    Some((h.remove(stemmed), stemmed.clone()))
                }
            };
            if let Some((e, stemmed)) = elem {
                let (p_pos, mut subvec) = match e {
                    None => (0, vec!()),
                    Some(y) => y
                };
                if p_pos != 0 && pos - p_pos < self.max_distance {
                    subvec.push(i);
                    let v = subvec.len() as f32;
                    for x in &subvec {
                        vec[*x].set_count(v);
                    }
                    h.insert(stemmed, (pos, subvec));
                } else {
                    subvec = vec!(i);
                    h.insert(stemmed, (pos, subvec));
                }
            }
        }
        vec
    }
    
    /// Detect the global number of repetitions.
    ///
    /// For each word, repetition value is set to the total number of occurences of this word in whole text.
    ///
    /// # Arguments
    ///
    /// * `vec` – A vector of `Word`.
    /// * `is_relative` – If true, divide the number of occurences by the number of words in the text.
    pub fn detect_global(&self, mut vec:Vec<Word>, is_relative: bool) -> Vec<Word> {
        let mut h:HashMap<String, f32> = HashMap::new();
        let mut count = 0;

        // first loop: we fill the map to count the occurences
        for i in 0 .. vec.len() {
            match &vec[i] {
                &Word::Untracked(_) => {}
                &Word::Ignored(_) => {count += 1;},
                &Word::Tracked(_, ref stemmed, _) => {
                    count += 1;
                    let x = match h.get(stemmed) {
                        None => 0.0,
                        Some(y) => *y
                    } + 1.0;
                    h.insert(stemmed.clone(), x);
                }
            };
        }
        // second loop: we set each word value to the number of
        // occurences
        for i in 0..vec.len() {
            let tmp = if let Word::Tracked(_, ref stemmed, _) = vec[i] {
                let x = h.get(stemmed).expect("HashMap was not filled correctly");
                Some(*x)
            } else {
                None
            };
            if let Some(x) = tmp {
                if is_relative {
                    vec[i].set_count(x / (count as f32));
                } else {
                    vec[i].set_count(x);
                }
            }
        }
        vec
    }

    /// Display the words to HTML.
    ///
    /// Use some basic CSS/Js for underlining repetitions and highlighting the
    /// over occurrences of the word under the mouse.
    ///
    /// # Arguments
    ///
    /// * `words` – A vector containing repetitions.
    /// * `threshold` – The threshold above which words must be highlighted.
    /// * `standalone` –  If true, generate a standalone HTML file.
    pub fn words_to_html(&self, words: &Vec<Word>, threshold: f32, standalone: bool) -> String {
        let mut res = String::new();
        if standalone {
            res = res + START;
        }
        
        for word in words {
            match word {
                &Word::Untracked(ref s) => res = res + s,
                &Word::Ignored(ref s) => res = res + s,
                &Word::Tracked(ref s, ref stemmed, x) => {
                    let this = format!("<span class = \"{}\" \
                                        onmouseover = 'on(\"{}\")' \
                                        onmouseout = 'off(\"{}\")' \
                                        {}>{}</span>",
                                       stemmed,
                                       stemmed,
                                       stemmed,
                                       value_to_style(x, threshold),
                                       s);
                    res = res + &this;
                }
            }
        }
        
        if standalone {
            res = res + END;
        }
        
        if !self.html {
            // If input is in HTML, don't add <br /> for newlines
            res.replace("\n", "<br/>\n")
        } else {
            res
        }
    }
}

/// Generate the style attribute according to x and threshold
fn value_to_style(x: f32, threshold: f32) -> &'static str {
    if x < threshold {
        ""
    } else if x < 1.5 * threshold {
        "style = \"text-decoration: underline; color: green;\""
    } else if x < 2.0 * threshold {
        "style = \"text-decoration: underline; color: orange;\""
    } else {
        "style = \"text-decoration: underline; color: red;\""
    }
}


