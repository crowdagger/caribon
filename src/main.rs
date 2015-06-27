extern crate caribon;
use caribon::Parser;

use std::io;
use std::io::Read;

fn main() {
    let parser = Parser::new("french")
        .unwrap()
        .with_html(true)
        .with_ignore_proper(true);
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    
//    let s = "Voici un petit texte afin de Détecter si ce détecteur de répétitions fonctionne et détecte bien les répétitions car les répétitions \
//c'est pas bien on veut pouvoir les détecter !";
    println!("{}", caribon::words_to_html(&parser.detect_local(parser.tokenize(&s)),
                                            1.9));
}
