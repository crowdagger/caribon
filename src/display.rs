use word::Word;

static START:&'static str = include_str!("html/start.html");
static END:&'static str = include_str!("html/end.html");

/// Display the words to HTML
/// It is then possible to use CSS/Js for highliht/whatever
///
/// # Arguments
///
/// *  `words` – A vector containing all words
/// * `threshold` – The threshold above which words must be highlighted
pub fn words_to_html(words: &Vec<Word>, threshold: f32) -> String {
    let mut res = String::new();
    res = res + START;

    for word in words {
        match word {
            &Word::Untracked(ref s) => res = res + s,
            &Word::Ignored(ref s) => res = res + s,
            &Word::Tracked(ref s, ref stemmed, x) => {
                let this = format!("<span class = \"{}\" \
                                    onmouseover = 'on(\"{}\")' \
                                    onmouseout = 'off(\"{}\")' \
                                    {}>{}</span>",
                                   stemmed,
                                   stemmed,
                                   stemmed,
                                   value_to_style(x, threshold),
                                   s);
                res = res + &this;
            }
        }
        res = res + " ";
    }
    res = res + END;
    res
}


/// Generate the style attribute according to x and threshold
fn value_to_style(x: f32, threshold: f32) -> &'static str {
    if x < threshold {
        ""
    } else if x < 2.0 * threshold {
        "style = \"text-decoration: underline; color: orange;\""
    } else {
        "style = \"text-decoration: underline; color: red;\""
    }
}
    
