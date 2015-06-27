extern crate stemmer;
use stemmer::Stemmer;

extern crate caribon;
use caribon::Word;
use caribon::Parser;

fn main() {
    let parser = Parser::new("french").unwrap();
    let ignored = ["la", "le", "et"];
    let parser = parser.with_ignored(&ignored);
    println!("{:?}", parser.ignored);
    let s = "le vent se lève";
    println!("{:?}", parser.tokenize(s));
}
