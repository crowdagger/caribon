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

use super::stemmer::Stemmer;
use super::edit_distance::edit_distance;
use word::{Word, Ast};
use error::{Error, Result};
use std::collections::HashMap;
use display::{get_shell_colour, value_to_colour, SHELL_COLOUR_OFF, SCRIPTS};

type TokenizeResult<'a> = Result<(&'a [char], Word)>;

const IGNORED_FR: &'static str = "la le les pas ne nos des ils elles il elle se on nous vous leur \
                                  leurs de et un une t s à d l je tu";
const IGNORED_EN: &'static str = "it s i of the a you we she he they them its their";

/// Parser which can load a string, detects repetition on it and outputs an HTML file.
#[repr(C)]
pub struct Parser {
    /// The stemmer 
    stemmer: Option<Stemmer>,
    /// List of ignored words: we don't want to count repetitions on them
    ignored: Vec<String>,
    /// Whether there is HTML in the input text
    html: bool,
    /// Ignores proper nouns
    ignore_proper: bool,
    /// Max distance to consider a repetition, only used for detect_local
    max_distance: u32,
    /// Triggers fuzzy string matching
    fuzzy: Option<f32>,
}

impl Parser {
    /// Returns a vector containing all languages that are implemented.
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
    ///
    /// # Example
    ///
    /// ```rust
    /// let v = caribon::Parser::get_ignored_from_string("some, words; to ignore");
    /// assert_eq!(v.len(), 4);
    /// ```
    pub fn get_ignored_from_string(list: &str) -> Vec<String> {
        list.split(|c: char| !c.is_alphabetic())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_owned())
            .collect()
    }

    /// Returns a vector containing the default ignored words for this language.
    pub fn get_ignored_from_lang(lang: &str) -> Vec<String> {
        match lang {
            "french" => Parser::get_ignored_from_string(IGNORED_FR),
            "english" => Parser::get_ignored_from_string(IGNORED_EN),
            _ => vec![],
        }
    }

    /// Returns `Ok(Parser)` if language is `ok`, Err(Error) else.
    ///
    /// # Arguments
    ///
    /// `lang` – The input text language. This will be used to create the
    ///          stemmer; it also determines what list of ignored words to use.
    ///          If `lang == "no_stemmer"`, stemming is disabled
    ///
    /// # Example
    ///
    /// ```
    /// let result = caribon::Parser::new("english");
    /// assert!(result.is_ok());
    /// ```
    ///
    /// ```
    /// let result = caribon::Parser::new("incorrect language");
    /// assert!(result.is_err());
    /// ```
    ///
    /// ```
    /// let result = caribon::Parser::new("no_stemmer");
    /// assert!(result.is_ok());
    /// ```
    pub fn new(lang: &str) -> Result<Parser> {
        let stemmer;
        if lang == "no_stemmer" {
            stemmer = None;
        } else {
            stemmer = Stemmer::new(lang);
            if stemmer.is_none() {
                return Err(Error {
                    content: format!("Language {} is not implemented.\nSupported languages: {}",
                                     lang,
                                     Parser::list_languages().join(", ")),
                });
            }
        }
        let ignored = Parser::get_ignored_from_lang(lang);
        Ok(Parser {
            stemmer: stemmer,
            ignored: ignored,
            html: true,
            ignore_proper: false,
            max_distance: 50,
            fuzzy: None,
        })
    }

    /// Sets fuzzy string matching (default None)
    ///
    /// If sets to Some(x), instead of just using equality to compare string,
    /// the Parser will use Levenhstein distance.
    ///
    /// # Arguments
    ///
    /// * `fuzzy` – `None` to deactivate fuzzy matching, or `Some(x)` to activate it. x must be between
    /// 0.0 and 1.0 as it corresponds to the relative distance, e.g "Caribon" has a length of 7 so if
    /// fuzzy is set with `Some(0.5)`, it will requires a maximal distance of 3 (actually 3.5 but distance is Integer)
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut parser = caribon::Parser::new("english").unwrap()
    ///                                             .with_fuzzy(Some(0.25));
    /// let mut ast = parser.tokenize("trust Rust").unwrap();
    /// parser.detect_local(&mut ast, 1.9);
    /// let result = parser.ast_to_markdown(&ast); // not the best output format, but easy to debug
    /// assert_eq!(&result, "**trust** **Rust**"); // these two words do have some letters in common
    /// ```
    pub fn with_fuzzy(mut self, fuzzy: Option<f32>) -> Parser {
        self.fuzzy = fuzzy;
        self
    }

    /// Sets max distance for repetitions (default 50).
    ///
    /// # Arguments
    ///
    /// `max_dist` – A number corresponding to a number of words. If two
    ///              occurences of a same word are separated by more than
    ///              this distance, it will not be counted as a repetition.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut parser = caribon::Parser::new("english").unwrap()
    ///                                             .with_max_distance(20);
    /// let mut ast = parser.tokenize("This word is repeated in a few words").unwrap();
    /// parser.detect_local(&mut ast, 1.9);
    /// let result = parser.ast_to_markdown(&ast); // not the best output format, but easy to debug
    /// assert_eq!(&result, "This **word** is repeated in a few **words**"); //repetition detected
    /// ```
    ///
    /// ```rust
    /// let mut parser = caribon::Parser::new("english").unwrap()
    ///                                             .with_max_distance(2);
    /// let mut ast = parser.tokenize("This word is repeated in a few words").unwrap();
    /// parser.detect_local(&mut ast, 1.9);
    /// let result = parser.ast_to_markdown(&ast); // not the best output format, but easy to debug
    /// assert_eq!(&result, "This word is repeated in a few words"); // repetition not detected because of
    ///                                                              // excessively low max_distance
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
    /// a sentence, but with most texts it won't be enough to highligth them as repetitions.
    pub fn with_ignore_proper(mut self, proper: bool) -> Parser {
        self.ignore_proper = proper;
        self
    }

    /// Sets the ignored list with a list of words contained in the argument string.
    ///
    /// This method *replaces* the default list of ignored words. If you want to *add*
    /// ignored words to the default list of a language, use `with_ignored` instead.
    ///
    /// # Arguments
    ///
    /// * `list` – A comma or whitespace separated list of words that should be ignored.
    pub fn with_ignored(mut self, list: &str) -> Parser {
        self.ignored = Parser::get_ignored_from_string(list);
        self
    }

    /// Appends a list of words contained in the argument string to the list of ignored words
    ///
    /// # Arguments
    ///
    /// * `list` – A comma or whitespace separated list of words that should be ignored.
    pub fn with_more_ignored(mut self, list: &str) -> Parser {
        list.split(|c: char| !c.is_alphabetic())
            .fold((), |_, s| {
                self.ignored.push(s.to_owned());
            });
        self
    }

    /// When we know it is the beginning of an escape character (e.g. &nbsp;)
    fn tokenize_escape<'b>(&self, c: &'b [char]) -> TokenizeResult<'b> {
        let mut res = String::new();
        let mut chars: &[char] = c;

        loop {
            if chars.is_empty() {
                return Err(Error::new("Error reading HTML: ill-formed escape code. Maybe this \
                                       is not an HTML file?"));
            }
            let c = chars[0];
            res.push(c);
            chars = &chars[1..];
            if c == ';' {
                return Ok((chars, Word::Untracked(res)));
            }
        }
    }

    fn tokenize_html<'b>(&self,
                         c: &'b [char],
                         ast: &mut Ast,
                         in_body: &mut bool)
                         -> TokenizeResult<'b> {
        let mut res = String::new();
        let mut chars: &[char] = c;
        let mut brackets = 1;
        let mut was_tag_found = false;

        res.push(chars[0]);
        chars = &chars[1..];

        // Inner function to determine if a char is part of a possible tag name
        fn is_tag(c: char) -> bool {
            c == '[' || c == ']' || c == '/' || c.is_alphabetic()
        }

        loop {
            if chars.is_empty() {
                return Err(Error {
                    content: format!("Error reading HTML: unclosed tag. Maybe this is not an HTML \
                                      file?
Details: the following was not closed: {}",
                                     res),
                });
            }
            let c = chars[0];
            res.push(c);
            if !was_tag_found && is_tag(c) {
                was_tag_found = true;

                let tag: String = chars.iter()
                                       .take_while(|c: &&char| is_tag(**c))
                                       .map(|c| c.to_lowercase().collect::<String>())
                                       .fold(String::new(), |acc, x| acc + &x);
                match &*tag {
                    "head" => {
                        ast.mark_begin_head();
                        *in_body = false;
                    }
                    "body" => {
                        ast.mark_begin_body();
                        *in_body = true;
                    }
                    "/body" => {
                        ast.mark_end_body();
                        *in_body = false;
                    }
                    "html" => {
                        *in_body = false;
                    }
                    "[cdata[" => {
                        // Special loop for CDATA
                        chars = &chars[1..];
                        loop {
                            if chars.len() < 2 {
                                return Err(Error::new("CDATA field not terminated properly"));
                            }
                            res.push(chars[0]);
                            if chars[0] == ']' && chars[1] == ']' {
                                res.push(chars[1]);
                                chars = &chars[2..];
                                break;
                            } else {
                                chars = &chars[1..];
                            }
                        }
                        continue;
                    }
                    _ => (),
                }
            }
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
        Ok((chars, Word::Untracked(res)))
    }

    fn tokenize_whitespace<'b>(&self, c: &'b [char], is_begin: &mut bool) -> TokenizeResult<'b> {
        let mut res = String::new();
        let mut chars: &[char] = c;

        loop {
            if chars.is_empty() {
                break;
            }
            let c = chars[0];
            if ((c == '<' || c == '&') && self.html) || c.is_alphabetic() {
                break;
            }
            chars = &chars[1..];
            res.push(c);
            if c == '.' {
                *is_begin = true;
            }
        }

        Ok((chars, Word::Untracked(res)))
    }

    /// Return true if `s` is a proper noun, false else
    fn is_proper_noun(&self, s: &str, is_begin: bool) -> bool {
        if self.ignore_proper {
            if !is_begin {
                let o = s.chars().next();
                match o {
                    None => false,
                    Some(c) => c.is_uppercase(),
                }
            } else {
                // Technically a proper noun could be at the beginning of a sentence :s
                false
            }
        } else {
            false
        }
    }

    fn tokenize_word<'b>(&mut self,
                         c: &'b [char],
                         is_begin: &mut bool,
                         in_body: bool)
                         -> TokenizeResult<'b> {
        let mut res = String::new();
        let mut chars: &[char] = c;

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

        let lower_s: Vec<String> = res.chars()
                                      .map(|c| c.to_lowercase().collect::<String>())
                                      .collect();
        let lower_s = lower_s.concat();
        let word = if !in_body {
            // We are not in body, so words are all untracked
            Word::Untracked(res)
        } else if self.ignored.contains(&lower_s) || self.is_proper_noun(&res, *is_begin) {
            Word::Ignored(res)
        } else {
            Word::Tracked(res, self.stem(&lower_s), 0.0, None)
        };

        *is_begin = false;
        Ok((chars, word))
    }


    /// Tokenize a string into a list of words. 
    ///
    /// This is the step that converts a string to some inner representation.
    ///
    /// # Arguments
    ///
    /// * `s` – The string to tokenize.
    pub fn tokenize(&mut self, s: &str) -> Result<Ast> {
        let v_chars: Vec<char> = s.chars().collect();
        let mut chars: &[char] = &v_chars;
        let mut ast = Ast::new();
        let mut is_sentence_beginning = true;
        let mut in_body = true;


        while !chars.is_empty() {
            let c = chars[0];
            let (cs, word) = if c.is_alphabetic() {
                try!(self.tokenize_word(chars, &mut is_sentence_beginning, in_body))
            } else if self.html && c == '<' {
                is_sentence_beginning = false;
                try!(self.tokenize_html(chars, &mut ast, &mut in_body))
            } else if self.html && c == '&' {
                try!(self.tokenize_escape(chars))
            } else {
                try!(self.tokenize_whitespace(chars, &mut is_sentence_beginning))
            };
            chars = cs;
            ast.words.push(word);
        }
        Ok(ast)
    }


    /// Detect the local number of repetitions.
    ///
    /// For each word, the repetition value is set to the total number of occurences of this word
    /// since there has been hat least `self.max_distance` between two occurences.
    ///
    /// It is the default algorithm, and probably the one you want to use.
    ///
    /// # Arguments
    ///
    /// `ast` – A mutable reference to an internal data structure returned by `tokenize`
    /// `threshold` – The threshold to consider a repetition (e.g. 1.9)
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut parser = caribon::Parser::new("english").unwrap();
    /// let mut ast = parser.tokenize("Testing whether this repetition detector works or does not work").unwrap();
    /// parser.detect_local(&mut ast, 1.9);
    /// let result = parser.ast_to_markdown(&ast); // not the most useful output format, but the easiest to debug
    /// assert_eq!(&result, "Testing whether this repetition detector **works** or does not **work**");
    /// ```
    pub fn detect_local(&self, ast: &mut Ast, threshold: f32) {
        let mut h: HashMap<String, (u32, Vec<usize>)> = HashMap::new();
        let mut pos: u32 = 1;
        let mut pos_to_i: Vec<usize> = vec![0];
        let mut vec = ast.get_body_mut();

        fn try_remove(pos: u32,
                      h: &mut HashMap<String, (u32, Vec<usize>)>,
                      vec: &[Word],
                      pos_to_i: &[usize],
                      max_distance: u32) {
            if pos > max_distance + 1 {
                let pos_limit = pos - max_distance;
                let i = pos_to_i[pos_limit as usize];
                let stemmed = match vec[i] {
                    Word::Untracked(_) => panic!("Should not happen"),
                    Word::Ignored(_) => return,
                    Word::Tracked(_, ref stemmed, _, _) => stemmed,
                };
                if let Some(&(old_pos, _)) = h.get(stemmed) {
                    if old_pos == pos_limit + 1 {
                        h.remove(stemmed);
                    }
                }
            }
        }
        for i in 0..vec.len() {
            let elem = match vec[i] {
                Word::Untracked(_) => None,
                Word::Ignored(_) => {
                    pos += 1;
                    pos_to_i.push(i);
                    None
                }
                Word::Tracked(_, ref stemmed, _, _) => {
                    pos += 1;
                    pos_to_i.push(i);
                    let s = self.fuzzy_get(&h, stemmed);
                    Some((h.remove(&s), s))
                }
            };
            // Try to remove elements on a map
            if self.fuzzy.is_some() {
                try_remove(pos, &mut h, vec, &pos_to_i, self.max_distance);
            }
            if let Some((e, stemmed)) = elem {
                // Update old stemmed to the fuzzy matched one
                vec[i].set_stemmed(stemmed.clone());
                let (p_pos, mut subvec) = match e {
                    None => (0, vec![]),
                    Some(y) => y,
                };
                if p_pos != 0 && pos - p_pos < self.max_distance {
                    subvec.push(i);
                    let v = subvec.len() as f32;
                    for x in &subvec {
                        vec[*x].set_count(v);
                    }
                    h.insert(stemmed, (pos, subvec));
                } else {
                    subvec = vec![i];
                    h.insert(stemmed, (pos, subvec));
                }
            }
        }
        self.highlight(vec, threshold, value_to_colour)
    }

    /// Returns stats about the words
    ///
    /// # Arguments
    ///
    /// `words` – A reference to a list of words
    ///
    /// # Returns
    ///
    /// This method retuns a tuple:
    /// * the first element is a hashmap between stemmed strings and the number of occurences of this word
    /// * the second oelement is the total number of (valid) words in the list (non counting whitespace, HTML tags...)
    pub fn words_stats(&self, ast: &Ast) -> (HashMap<String, f32>, u32) {
        let mut h: HashMap<String, f32> = HashMap::new();
        let mut count: u32 = 0;
        let words: &[Word] = ast.get_body();

        // we fill the map and count
        for word in words {
            match *word {
                Word::Untracked(_) => {}
                Word::Ignored(_) => {
                    count += 1;
                }
                Word::Tracked(_, ref stemmed, _, _) => {
                    count += 1;
                    let x = match h.get(stemmed) {
                        None => 0.0,
                        Some(y) => *y,
                    } + 1.0;
                    h.insert(stemmed.clone(), x);
                }
            };
        }

        (h, count)
    }

    /// Detect the global number of repetitions.
    ///
    /// For each word, repetition value is set to the total number of occurences of this word in whole text,
    /// divided by total number of words in the text
    ///
    /// # Arguments
    ///
    /// * `vec` – A vector of `Word`.
    /// * `threshold` – A threshold to highlight repetitions (e.g. 0.01)
    pub fn detect_global(&self, ast: &mut Ast, threshold: f32) {
        let (h, count) = self.words_stats(&ast);
        let mut vec = ast.get_body_mut();


        // If there are not enough words for the threshold, do nothing instead of
        // underlining all words
        if 1.0 / threshold >= (count as f32) {
            return;
        }

        // We set each word value to the relative number of occurences
        for i in 0..vec.len() {
            let tmp = if let Word::Tracked(_, ref stemmed, _, _) = vec[i] {
                let x = h.get(stemmed).expect("HashMap was not filled correctly");
                Some(*x)
            } else {
                None
            };
            if let Some(x) = tmp {
                vec[i].set_count(x / (count as f32));

            }
        }
        self.highlight(vec, threshold, |_, _| "blue")
    }

    /// Highlight words whose value is superior to the threshold
    ///
    /// # Arguments
    ///
    /// * `words` – A vector containing repetitions.
    /// * `threshold` – The threshold above which words must be highlighted.
    /// * `f` – A closure that returns the colour given the value and threshold
    ///
    /// # Returns
    ///
    /// A vector of highlight
    fn highlight<F>(&self, words: &mut [Word], threshold: f32, f: F)
        where F: Fn(f32, f32) -> &'static str
    {
        let mut res = words;
        for i in 0..res.len() {
            let word: &mut Word = &mut res[i];
            if let Word::Tracked(_, _, ref mut v, ref mut option) = *word {
                if option.is_none() && *v >= threshold {
                    // No colour is attributed, so see if we attribute one
                    *option = Some(f(*v, threshold));
                }
                *v = 0.0; //resets value to zero so it doesn't cause problem if another algorithm is used after
            }
        }
    }

    /// Display the words to terminal, higlighting the repetitions.
    ///
    /// Use terminal colour codes to highlight the repetitions
    ///
    /// # Arguments
    ///
    /// * `ast` – A reference to `Ast`, returned by `tokenize` and modified by `detect_*`
    pub fn ast_to_terminal(&self, ast: &Ast) -> String {
        let mut res = String::new();
        let words = &ast.words;

        for word in words {
            match *word {
                Word::Untracked(ref s) => res = res + s,
                Word::Ignored(ref s) => res = res + s,
                Word::Tracked(ref s, _, _, option) => {
                    if let Some(colour) = option {
                        match get_shell_colour(colour) {
                            None => res = res + s,
                            Some(shell_colour) => res = res + shell_colour + s + SHELL_COLOUR_OFF,
                        }
                    } else {
                        res.push_str(s);
                    }
                }
            }
        }
        res
    }


    /// Display the Ast to markdown, emphasizing the repetitions.
    ///
    /// This is more limited than HTML or even terminal output, as it completely discards
    /// colour information that have been passed by `detect_*` methods, but it might be useful
    /// if e.g. you want to generate some files later with Pandoc (or any other program).
    ///
    /// # Arguments
    ///
    /// * `ast` – An Ast containing repetitions.
    pub fn ast_to_markdown(&self, ast: &Ast) -> String {
        let mut res = String::new();
        let words = &ast.words;

        for word in words {
            match word {
                &Word::Untracked(ref s) => res = res + s,
                &Word::Ignored(ref s) => res = res + s,
                &Word::Tracked(ref s, _, _, highlight) => {
                    if let Some(_) = highlight {
                        res.push_str("**");
                        res.push_str(s);
                        res.push_str("**");
                    } else {
                        res.push_str(s);
                    }
                }
            }
        }
        res
    }


    /// Display the Ast to HTML, higlighting the repetitions.
    ///
    /// Use some basic CSS/Js for underlining repetitions and highlighting the
    /// over occurrences of the word under the mouse.
    ///
    /// # Arguments
    ///
    /// * `ast` – An Ast containing repetitions.
    /// * `standalone` –  If true, generate a standalone HTML file, else just an HTML fragment
    pub fn ast_to_html(&self, ast: &mut Ast, standalone: bool) -> String {
        let mut res = String::new();
        let words: &[Word];

        // If standalone, only use words located between <body> and </body>
        if !standalone {
            // If standalone, only prints the body part of the AST
            words = ast.get_body();
        } else {
            // There is a head, so we must insert the scripts in the right place
            if let Some(i) = ast.begin_head {
                ast.words.insert(i + 1, Word::Untracked(SCRIPTS.to_owned()));
            } else {
                // If there is no head, generate the beginning of the document
                res.push_str("<html><head>\n");
                res.push_str("<meta charset = \"UTF-8\">\n");
                res.push_str(SCRIPTS);
                res.push_str("</head>\n");
                if ast.begin_body.is_none() || ast.end_body.is_none() {
                    res.push_str("<body>\n");
                }
            }
            words = &ast.words;
        }

        for word in words {
            match *word {
                Word::Untracked(ref s) => res = res + s,
                Word::Ignored(ref s) => res = res + s,
                Word::Tracked(ref s, ref stemmed, _, option) => {
                    let this = format!("<span class = \"{}\" onmouseover = 'on(\"{}\")' \
                                        onmouseout = 'off(\"{}\")' {}>{}</span>",
                                       stemmed,
                                       stemmed,
                                       stemmed,
                                       if let Some(colour) = option {
                                           format!("style = \"text-decoration: underline; color: \
                                                    {};\"",
                                                   colour)
                                       } else {
                                           String::new()
                                       },
                                       s);
                    res.push_str(&this);
                }
            }
        }


        if !self.html {
            // If input is in text, add <br /> for newlines
            res = res.replace("\n", "<br/>\n");
        }
        if standalone && ast.begin_body.is_none() && ast.end_body.is_none() {
            // We need to add </body> at the end
            res.push_str("</body></html>");
        }
        res
    }

    /// Stems a string
    ///
    /// Either warps call to `stemmer.stem`, or, if `stemmer == None`, just returns the string
    fn stem(&mut self, s: &str) -> String {
        match self.stemmer {
            Some(ref mut stemmer) => stemmer.stem(s),
            None => s.to_owned(),
        }
    }


    /// Search a string in a hashmap with fuzzy string matching
    /// Returns the matching string, or `None`
    fn fuzzy_get<T>(&self, h: &HashMap<String, T>, pattern: &str) -> String {
        if let Some(d_max) = self.fuzzy {
            let length = pattern.len();
            if length < 2 {
                // Pattern is too short to do fuzzy matching
                pattern.to_owned()
            } else {
                // If hashmap contains the exact pattern, no need to fuzzy search
                if h.contains_key(pattern) {
                    pattern.to_owned()
                } else {
                    let mut min_distance = pattern.len() as i32;
                    let mut key = pattern;
                    for s in h.keys()
                              .filter(|s| {
                                  // string is too small
                                  if s.len() < 2 {
                                      return false;
                                  }
                                  if (s.len() as f32 - length as f32).abs() >
                                     (d_max * pattern.len() as f32) {
                                      // Lengths don't allow a matching distance
                                      return false;
                                  }
                                  true
                              }) {
                        let dist = edit_distance(s, pattern);
                        if dist < min_distance {
                            min_distance = dist;
                            key = s;
                        }
                        if min_distance == 1 {
                            break; // best result since perfect match has been ruled out
                        }
                    }
                    if min_distance <= (d_max * pattern.len() as f32) as i32 {
                        key.to_owned()
                    } else {
                        pattern.to_owned()
                    }
                }
            }
        } else {
            pattern.to_owned()
        }
    }
}
