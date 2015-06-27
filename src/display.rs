use word::Word;

/// Display the words to HTML
/// It is then possible to use CSS/Js for highliht/whatever
///
/// # Arguments
///
/// *  `words` – A vector containing all words
/// * `threshold` – The threshold above which words must be highlighted
pub fn words_to_html(words: &Vec<Word>, threshold: f32) -> String {
    let mut res = String::new();

    for word in words {
        match word {
            &Word::Untracked(ref s) => res = res + s,
            &Word::Ignored(ref s) => res = res + s,
            &Word::Tracked(ref s, ref stemmed, x) => {
                let this = format!("<span class = \"{}\" {}>{}</span>",
                                   stemmed,
                                   if x >= threshold {
                                       "style = \"text-decoration: underline\""
                                   } else {
                                       ""
                                   },
                                   s);
                res = res + &this;
            }
        }
        res = res + " ";
    }
    res
}
