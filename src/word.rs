/// `Word` type.
pub enum Word<'a> {
    /// Untracked string (typically whitepsaced, HTML formatting, ...)
    Untracked(&'a str),
    /// Tracked string, containing the string, the stemmed variant of the
    /// string, and the degree of repetitions
    Tracked(&'a str, &'a str, f32)
}
