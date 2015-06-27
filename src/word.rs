/// `Word` type.
#[derive(Debug,Clone)]
pub enum Word {
    /// String which is not part of the text (typically whitepsaced, HTML formatting, ...)
    Untracked(String),
    /// Ignored word
    Ignored(String),
    /// Tracked string, containing the string, the stemmed variant of the
    /// string, and the degree of repetitions
    Tracked(String, String, f32)
}

impl Word {
    pub fn set_count(&mut self, x: f32) 
    {
        match self {
            &mut Word::Tracked(_, _, ref mut v) => {
                *v = x;
            },
            _ => {}
        }
    }
}
