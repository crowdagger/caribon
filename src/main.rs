extern crate stemmer;
use stemmer::Stemmer;

fn main() {
    let stemmer = Stemmer::new("french").unwrap();
    println!("{}", stemmer.stem("mangerai"));
}
