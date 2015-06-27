extern crate stemmer;
use self::stemmer::Stemmer;
use word::Word;

static IGNORED_FR:[&'static str; 17] = ["la", "le", "les", "pas", "ne",
                                       "nos", "des", "ils", "elles", "il",
                                       "elle", "se", "on", "nous", "vous",
                                       "leur", "leurs"];
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
    pub fn tokenize(&self, s: &str) -> Vec<Word> {
        let vec:Vec<&str> = s.split_whitespace().collect();
        let mut res = vec!();
        for s in vec {
            if self.ignored.contains(&s) {
                res.push(Word::Untracked(s.to_string()));
            } else {
                res.push(Word::Tracked(s.to_string(), self.stemmer.stem(s), 0.0));
            }
        }
        res
    }
}

