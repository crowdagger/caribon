/// `Word` type.
#[derive(Debug)]
pub enum Word {
    /// Untracked string (typically whitepsaced, HTML formatting, ...)
    Untracked(String),
    /// Tracked string, containing the string, the stemmed variant of the
    /// string, and the degree of repetitions
    Tracked(String, String, f32)
}
