use super::Parser;
extern crate test;
use self::test::Bencher;

static TEST:&'static str = "This is some text string. We want to detect repetitions in it.
Why? Because repetitions are bad, very bad, so we want to highlight them. In order to have benches 
that are not utterly worthless, it is required to have some variance in the text, so even if we are gonna repeat
this string quite a while it must be a bit longer, so I am writing senseless stuff. I guess this is long enough now ? 
Well, alright, let's say it is. ";
static N_REPET:u32 = 500;

fn get_input() -> String {
    let mut s = TEST.to_string();
    for _ in 1..N_REPET {
        s = s + TEST;
    }

    s
}

#[bench]
fn bench_clone(b:&mut Bencher) {
    let s = get_input();
    let parser = Parser::new("english").unwrap();
    let words = parser.tokenize(&s).unwrap();
    b.iter(|| {
        words.clone();
    });
}


#[bench]
fn bench_html(b:&mut Bencher) {
    let s = get_input();
    let parser = Parser::new("english").unwrap().with_html(true);
    let words = parser.tokenize(&s).unwrap();
    let repetitions = parser.detect_local(words.clone(), 2.0);
    b.iter(|| {
        parser.words_to_html(&repetitions, false);
    });
}

#[bench]
fn bench_html2(b:&mut Bencher) {
    let s = get_input();
    let parser = Parser::new("english").unwrap().with_html(false);
    let words = parser.tokenize(&s).unwrap();
    let repetitions = parser.detect_local(words.clone(), 2.0);
    b.iter(|| {
        parser.words_to_html(&repetitions, false);
    });
}

#[bench]
fn bench_terminal(b:&mut Bencher) {
    let s = get_input();
    let parser = Parser::new("english").unwrap();
    let words = parser.tokenize(&s).unwrap();
    let repetitions = parser.detect_local(words.clone(), 2.0);
    b.iter(|| {
        parser.words_to_terminal(&repetitions);
    });
}

#[bench]
fn bench_markdown(b:&mut Bencher) {
    let s = get_input();
    let parser = Parser::new("english").unwrap();
    let words = parser.tokenize(&s).unwrap();
    let repetitions = parser.detect_local(words.clone(), 2.0);
    b.iter(|| {
        parser.words_to_markdown(&repetitions);
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
//    let s = include_str!("../README.md");
    let parser = Parser::new("english").unwrap();
    let words = parser.tokenize(&s).unwrap();    
    b.iter(|| {
        parser.detect_local(words.clone(), 1.9);
    });
}

#[bench]
fn bench_local_fuzzy(b:&mut Bencher) {
    let s = get_input();
    let parser = Parser::new("english").unwrap().with_fuzzy(Some(0.5));
    let words = parser.tokenize(&s).unwrap();    
    b.iter(|| {
        parser.detect_local(words.clone(), 1.9);
    });
}

#[bench]
fn bench_local2(b:&mut Bencher) {
    let s = get_input();
//    let s = include_str!("../README.md");
    let parser = Parser::new("english").unwrap();
    let words = parser.tokenize(&s).unwrap();    
    b.iter(|| {
        parser.detect_local2(words.clone(), 1.9);
    });
}

#[bench]
fn bench_local2_fuzzy(b:&mut Bencher) {
    let s = get_input();
    let parser = Parser::new("english").unwrap().with_fuzzy(Some(0.5));
    let words = parser.tokenize(&s).unwrap();    
    b.iter(|| {
        parser.detect_local2(words.clone(), 1.9);
    });
}


#[bench]
fn bench_total(b:&mut Bencher) {
    let s = get_input();
    b.iter(|| {
        let parser = Parser::new("english").unwrap().with_fuzzy(Some(0.5));
        let words = parser.tokenize(&s).unwrap();    
        let detections = parser.detect_local(words.clone(), 1.9);
        parser.words_to_html(&detections, true);
    });
}


#[bench]
fn bench_local_readme(b:&mut Bencher) {
    //    let s = get_input();
    let s = include_str!("../README.md");
    let parser = Parser::new("english").unwrap();
    let words = parser.tokenize(&s).unwrap();    
    b.iter(|| {
        parser.detect_local(words.clone(), 1.9);
    });
}

#[bench]
fn bench_local_readme_fuzzy(b:&mut Bencher) {
    //    let s = get_input();
    let s = include_str!("../README.md");
    let parser = Parser::new("english").unwrap().with_fuzzy(Some(0.5));
    let words = parser.tokenize(&s).unwrap();    
    b.iter(|| {
        parser.detect_local(words.clone(), 1.9);
    });
}

#[bench]
fn bench_local2_readme(b:&mut Bencher) {
    //    let s = get_input();
    let s = include_str!("../README.md");
    let parser = Parser::new("english").unwrap();
    let words = parser.tokenize(&s).unwrap();    
    b.iter(|| {
        parser.detect_local2(words.clone(), 1.9);
    });
}

#[bench]
fn bench_local2_readme_fuzzy(b:&mut Bencher) {
    //    let s = get_input();
    let s = include_str!("../README.md");
    let parser = Parser::new("english").unwrap().with_fuzzy(Some(0.5));
    let words = parser.tokenize(&s).unwrap();    
    b.iter(|| {
        parser.detect_local2(words.clone(), 1.9);
    });
}

#[bench]
fn bench_leak_readme(b:&mut Bencher) {
    //    let s = get_input();
    let s = include_str!("../README.md");
    let parser = Parser::new("english").unwrap();
    let words = parser.tokenize(&s).unwrap();    
    b.iter(|| {
        parser.detect_leak(words.clone(), 1.9);
    });
}

#[bench]
fn bench_leak_readme_fuzzy(b:&mut Bencher) {
    //    let s = get_input();
    let s = include_str!("../README.md");
    let parser = Parser::new("english").unwrap().with_fuzzy(Some(0.5));
    let words = parser.tokenize(&s).unwrap();    
    b.iter(|| {
        parser.detect_leak(words.clone(), 1.9);
    });
}

#[bench]
fn bench_global_readme(b:&mut Bencher) {
    //    let s = get_input();
    let s = include_str!("../README.md");
    let parser = Parser::new("english").unwrap();
    let words = parser.tokenize(&s).unwrap();    
    b.iter(|| {
        parser.detect_global(words.clone(), 0.01);
    });
}

#[bench]
fn bench_global_readme_fuzzy(b:&mut Bencher) {
    //    let s = get_input();
    let s = include_str!("../README.md");
    let parser = Parser::new("english").unwrap().with_fuzzy(Some(0.5));
    let words = parser.tokenize(&s).unwrap();    
    b.iter(|| {
        parser.detect_global(words.clone(), 0.01);
    });
}


#[bench]
fn bench_local10(b:&mut Bencher) {
    let s = get_input();
    let parser = Parser::new("english").unwrap().with_max_distance(10);
    let words = parser.tokenize(&s).unwrap();
    b.iter(|| {
        parser.detect_local(words.clone(), 1.9);
    });
}

#[bench]
fn bench_local100(b:&mut Bencher) {
    let s = get_input();
    let parser = Parser::new("english").unwrap().with_max_distance(100);
    let words = parser.tokenize(&s).unwrap();
    b.iter(|| {
        parser.detect_local(words.clone(), 1.9);
    });
}

#[bench]
fn bench_global(b:&mut Bencher) {
    let s = get_input();
    let parser = Parser::new("english").unwrap();
    let words = parser.tokenize(&s).unwrap();
    b.iter(|| {
        parser.detect_global(words.clone(), 0.01);
    });
}

#[bench]
fn bench_leak(b:&mut Bencher) {
    let s = get_input();
    let parser = Parser::new("english").unwrap();
    let words = parser.tokenize(&s).unwrap();
    b.iter(|| {
        parser.detect_leak(words.clone(), 1.5);
    });
}

