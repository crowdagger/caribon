use super::Parser;
extern crate test;
use self::test::Bencher;

static TEST:&'static str = "This is some text string. We want to detect repetitions in it.
Why? Because repetitions are bad, very bad, so we want to highlight them. ";
static N_REPET:u32 = 100;

fn get_input() -> String {
    let mut s = TEST.to_string();
    for _ in 1..N_REPET {
        s = s + TEST;
    }
    s
}


#[bench]
fn bench_html(b:&mut Bencher) {
    let s = get_input();
    let parser = Parser::new("english").unwrap();
    let words = parser.tokenize(&s).unwrap();
    let repetitions = parser.detect_local(words.clone());
    b.iter(|| {
        let html = parser.words_to_html(&repetitions, 2.0, false);
    });
}

#[bench]
fn bench_tokenize(b:&mut Bencher) {
    let s = get_input();
    let parser = Parser::new("english").unwrap();
    b.iter(|| {
        parser.tokenize(&s).unwrap();
    });
}


#[bench]
fn bench_local(b:&mut Bencher) {
    let s = get_input();
    let parser = Parser::new("english").unwrap();
    let words = parser.tokenize(&s).unwrap();    
    b.iter(|| {
        parser.detect_local(words.clone());
    });
}


#[bench]
fn bench_local10(b:&mut Bencher) {
    let s = get_input();
    let parser = Parser::new("english").unwrap().with_max_distance(10);
    let words = parser.tokenize(&s).unwrap();
    b.iter(|| {
        parser.detect_local(words.clone());
    });
}

#[bench]
fn bench_local100(b:&mut Bencher) {
    let s = get_input();
    let parser = Parser::new("english").unwrap().with_max_distance(100);
    let words = parser.tokenize(&s).unwrap();
    b.iter(|| {
        parser.detect_local(words.clone());
    });
}

#[bench]
fn bench_global(b:&mut Bencher) {
    let s = get_input();
    let parser = Parser::new("english").unwrap();
    let words = parser.tokenize(&s).unwrap();
    b.iter(|| {
        parser.detect_global(words.clone(), true);
    });
}

#[bench]
fn bench_leak(b:&mut Bencher) {
    let s = get_input();
    let parser = Parser::new("english").unwrap();
    let words = parser.tokenize(&s).unwrap();
    b.iter(|| {
        parser.detect_leak(words.clone());
    });
}

