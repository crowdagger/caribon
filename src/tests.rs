use super::Parser;
extern crate test;
use self::test::Bencher;

static TEST:&'static str = "This is some text string. We want to detect repetitions in it.
Why? Because repetitions are bad, very bad, so we want to highlight them. ";

#[bench]
fn bench_local(b:&mut Bencher) {
    let mut s = TEST.to_string();
    for _ in 0..2000 {
        s = s + TEST;
    }
    b.iter(|| {
        let parser = Parser::new("english").unwrap();
        let words = parser.tokenize(&s);
        let repetitions = parser.detect_local(words);
        let html = parser.words_to_html(&repetitions, 2.0, false);
    });
}

#[bench]
fn bench_global(b:&mut Bencher) {
    let mut s = TEST.to_string();
    for _ in 0..2000 {
        s = s + TEST;
    }
    b.iter(|| {
        let parser = Parser::new("english").unwrap();
        let words = parser.tokenize(&s);
        let repetitions = parser.detect_global(words, true);
        let html = parser.words_to_html(&repetitions, 2.0, false);
    });
}

#[bench]
fn bench_leak(b:&mut Bencher) {
    let mut s = TEST.to_string();
    for _ in 0..2000 {
        s = s + TEST;
    }
    b.iter(|| {
        let parser = Parser::new("english").unwrap();
        let words = parser.tokenize(&s);
        let repetitions = parser.detect_leak(words);
        let html = parser.words_to_html(&repetitions, 2.0, false);
    });
}

