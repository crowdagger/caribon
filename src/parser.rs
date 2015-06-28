extern crate stemmer;
use self::stemmer::Stemmer;
use word::Word;
use std::collections::HashMap;

static IGNORED_FR:[&'static str; 26] = ["la", "le", "les", "pas", "ne",
                                       "nos", "des", "ils", "elles", "il",
                                       "elle", "se", "on", "nous", "vous",
                                        "leur", "leurs", "de", "et", "un",
                                        "une", "t", "s", "à", "d",
                                        "l"];
static IGNORED_DEFAULT:[&'static str; 0] = [];

/// Parser type
pub struct Parser<'a> {
    /// The stemmer object
    stemmer: Stemmer,
    /// List of ignored words
    ignored: &'a[&'a str],
    /// Detect HTML in input
    html: bool,
    /// Ignores proper nouns
    ignore_proper: bool,
    /// Leak, only used for detect_leak
    leak: f32,
    /// Max distance to consider a repetition, only used for detect_local
    max_distance: u32
}

impl<'a> Parser<'a> {
    ///  Returns a vector containing all languages implemented
    pub fn list_languages() -> Vec<&'static str> {
        Stemmer::list()
    }
    
    /// Returns Some(Parser) if language is ok, None else
    pub fn new(lang: &str) -> Option<Parser> {
        let stemmer = Stemmer::new(lang);
        if stemmer.is_none() {
            return None;
        }
        let stemmer = stemmer.unwrap();
        let ignored:&'static[&'static str] = match lang {
            "french" => &IGNORED_FR,
            _ => &IGNORED_DEFAULT
        };
        Some(Parser{stemmer: stemmer,
                    ignored: ignored,
                    html: true,
                    ignore_proper: false,
                    leak: 0.98,
                    max_distance: 50})
    }

    /// Sets max distance for repetition (default 50)
    pub fn with_max_distance(mut self, max_dist: u32) -> Parser<'a> {
        self.max_distance = max_dist;
        self
    }
    
    /// Sets HTML detection in input (default true)
    pub fn with_html(mut self, html: bool) -> Parser<'a> {
        self.html = html;
        self
    }

    /// Sets ignore_proper flag (default false)
    pub fn with_ignore_proper(mut self, proper: bool) -> Parser<'a> {
        self.ignore_proper = proper;
        self
    }
    
    /// Sets the leak (default 0.98)
    pub fn with_leak(mut self, leak: f32) -> Parser<'a> {
        self.leak = leak;
        self
    }
        
    /// Sets the ignored keyword
    pub fn with_ignored(mut self, ignored: &'a [&'a str]) -> Parser<'a> {
        self.ignored = ignored;
        self
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
            if c == '<' || c.is_alphabetic() {
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
        let word = if self.ignored.contains(&&*lower_s)
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


    /// Tokenize a string into a list of words
    ///
    /// # Arguments
    ///
    /// * `s` – The string to tokenize
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
            } else {
                let (c, word) = self.tokenize_whitespace(chars, &mut is_sentence_beginning);
                chars = c;
                res.push(word);
            }
        }
     
        res
    }

    /// Detect the local repetitions, using a leak value
    ///
    /// Basically, each time a word occurs, increase value by 1.0
    /// and each time it does not, multiply by leak (default: 0.98)
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
    /// For each word, value is set to the total number of occurences of this word
    /// but it is reset to zero if there is more than `self.max_distance` between
    /// two occurences.
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
    /// For each word, value is set to the total number of occurences of this word.
    ///
    /// # Arguments
    ///
    /// * `vec` – A vector of `Word`
    /// * `is_relative` – If true, divide the number of occurences by total number of words
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
}

