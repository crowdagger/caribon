extern crate stemmer;
use stemmer::Stemmer;

extern crate caribon;
use caribon::Word;
use caribon::Parser;

fn main() {
    let parser = Parser::new("french").unwrap();
    let ignored = ["la", "le", "et"];
//    let parser = parser.with_ignored(&ignored);
//    println!("{:?}", parser.ignored);
    let s = "Voici un petit texte afin de Détecter si ce détecteur de répétitions fonctionne et détecte bien les répétitions car les répétitions \
c'est pas bien on veut pouvoir les détecter !";
    println!("{}", caribon::words_to_html(&parser.detect_global(parser.tokenize(s)),
                                            1.4));
}
