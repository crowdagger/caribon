extern crate stemmer;
use self::stemmer::Stemmer;
use word;
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
    stemmer: Stemmer,
    pub ignored: &'a[&'a str]
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
                    ignored: ignored})
    }

    /// Sets the ignored keyword
    pub fn with_ignored(mut self, ignored: &'a [&'a str]) -> Parser<'a> {
        self.ignored = ignored;
        self
    }

    /// Tokenize a string into a list of words
    fn tokenize(&self, s: &str) -> Vec<Word> {
        let vec:Vec<&str> = s.split_whitespace().collect();
        let mut res = vec!();
        for s in vec {
            if self.ignored.contains(&s) {
                res.push(Word::Ignored(s.to_string()));
            } else {
                res.push(Word::Tracked(word::Tracking{content:s.to_string(),
                                                      stemmed: self.stemmer.stem(s),
                                                      value: 0.0}));
            }
        }
        res
    }

    /// Parse a string
    pub fn parse(&self, s:&str) -> Vec<Word> {
        let mut vec = self.tokenize(s);
        let mut h:HashMap<String, f32> = HashMap::new();
        let mut pos = 0;

        for i in 0 .. vec.len() {
            let v = match &vec[i] {
                &Word::Untracked(_) => None,
                &Word::Ignored(_) => {
                    pos += 1;
                    None
                },
                &Word::Tracked(ref tracking) => {
                    let x = match h.get(&tracking.stemmed) {
                        None => 0.0,
                        Some(y) => *y
                    } + 1.0;
                    h.insert(tracking.stemmed.clone(), x);
                    pos += 1;
                    Some(x)
                }
            };
            match v {
                None => {},
                Some(x) => {
                    vec[i] = vec[i].clone().set_count(x);
                }
            }
        }
        vec
    }
}

