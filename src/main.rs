extern crate stemmer;
use stemmer::Stemmer;

extern crate caribon;
use caribon::Word;
use caribon::Parser;

fn main() {
    let parser = Parser::new("french").unwrap();
    let ignored = ["la", "le", "et"];
//    let parser = parser.with_ignored(&ignored);
    println!("{:?}", parser.ignored);
    let s = "voici un petit texte afin de détecter si ce détecteur de répétitions fonctionne et détecte bien les répétitions car les répétitions \
c'est pas bien on veut pouvoir les détecter !";
    println!("{:?}", parser.detect_leak(parser.tokenize(s)));
}
