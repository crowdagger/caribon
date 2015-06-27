extern crate stemmer;
use self::stemmer::Stemmer;
use word::Word;
use std::collections::HashMap;

static IGNORED_FR:[&'static str; 21] = ["la", "le", "les", "pas", "ne",
                                       "nos", "des", "ils", "elles", "il",
                                       "elle", "se", "on", "nous", "vous",
                                        "leur", "leurs", "de", "et", "un",
                                        "une"];
static IGNORED_DEFAULT:[&'static str; 0] = [];

/// Parser type
pub struct Parser<'a> {
    /// The stemmer object
    stemmer: Stemmer,
    /// List of ignored words
    pub ignored: &'a[&'a str],
    /// Leak, only used for some algorithms
    leak: f32
}

impl<'a> Parser<'a> {
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
                    leak: 0.98})
    }

    /// Sets the leak    
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
    
    fn tokenize_whitespace<'b>(&self, c:&'b [char]) -> (&'b [char], Word) {
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
        }

        (chars, Word::Untracked(res))
    }
    

    fn tokenize_word<'b>(&self, c: &'b [char]) -> (&'b [char], Word) {
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
        let word = if self.ignored.contains(&&*lower_s) {
            Word::Ignored(res.to_string())
        } else {
            Word::Tracked(res.to_string(),
                          self.stemmer.stem(&lower_s),
                          0.0)
        };

        (chars, word)
    }


    /// Tokenize a string into a list of words
    ///
    /// # Arguments
    ///
    /// * `s` – The string to tokenize
    /// * `html` – If true, escapes html content
    pub fn tokenize(&self, s: &str, html: bool) -> Vec<Word> {
        let v_chars:Vec<char> = s.chars().collect();
        let mut chars:&[char] = &v_chars;
        let mut res:Vec<Word> = vec!();

        while !chars.is_empty() {
            let c = chars[0];
            if c.is_alphabetic() {
                let (c, word) = self.tokenize_word(chars);
                chars = c;
                res.push(word);
            } else if html && c == '<' {
                let (c, word) = self.tokenize_html(chars);
                chars = c;
                res.push(word);
            } else {
                let (c, word) = self.tokenize_whitespace(chars);
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
    

    /// Detect the global number of repetitions.
    ///
    /// For each word, value is set to the total number of occurences of this word.
    pub fn detect_global(&self, mut vec:Vec<Word>) -> Vec<Word> {
        let mut h:HashMap<String, f32> = HashMap::new();

        // first loop: we fill the map to count the occurences
        for i in 0 .. vec.len() {
            match &vec[i] {
                &Word::Untracked(_) => {}
                &Word::Ignored(_) => {},
                &Word::Tracked(_, ref stemmed, _) => {
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
                vec[i].set_count(x);
            }
        }
        vec
    }
}

