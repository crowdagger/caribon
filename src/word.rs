#[derive(Debug,Clone)]
pub struct Tracking {
    pub content: String,
    pub stemmed: String,
    pub value: f32
}

/// `Word` type.
#[derive(Debug,Clone)]
pub enum Word {
    /// String which is not part of the text (typically whitepsaced, HTML formatting, ...)
    Untracked(String),
    /// Ignored word
    Ignored(String),
    /// Tracked string, containing the string, the stemmed variant of the
    /// string, and the degree of repetitions
    Tracked(Tracking)
}

impl Word {
    pub fn set_count(mut self, x: f32) -> Word
    {
        match self {
            Word::Tracked(mut tracking) => {
                tracking.value = x;
                Word::Tracked(tracking)
            },
            _ => self
        }
    }
}
