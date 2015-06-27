extern crate stemmer;
use self::stemmer::Stemmer;
use word::Word;

/// Parser type
pub struct Parser {
    stemmer: Stemmer
}

impl Parser {
    /// Returns Some(Parser) if language is ok, None else
    pub fn new(lang: &str) -> Option<Parser> {
        let stemmer = Stemmer::new(lang);
        match stemmer {
            Some(s) => Some(Parser{stemmer: s}),
            None => None
        }
    }
}

