// Copyright (C) 2015 Élisabeth HENRY.
//
// This file is part of Caribon.
//
// Caribon is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published
// by the Free Software Foundation, either version 2.1 of the License, or
// (at your option) any later version.
//
// Caribon is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with Caribon.  If not, see <http://www.gnu.org/licenses/>.

use super::Parser;
extern crate test;
use self::test::Bencher;

static TEST:&'static str = "This is some text string. We want to detect repetitions in it.
Why? Because repetitions are bad, very bad, so we want to highlight them. In order to have benches 
that are not utterly worthless, it is required to have some variance in the text, so even if we are gonna repeat
this string quite a while it must be a bit longer, so I am writing senseless stuff. I guess this is long enough now ? 
Well, alright, let's say it is. ";
static N_REPET:u32 = 100;

fn get_input() -> String {
    let mut s = TEST.to_owned();
    for _ in 1..N_REPET {
        s.push_str(TEST);
    }

    s
}

#[bench]
fn bench_clone(b:&mut Bencher) {
    let s = get_input();
    let mut parser = Parser::new("english").unwrap();
    let ast = parser.tokenize(&s).unwrap();
    b.iter(|| {
        ast.clone();
    });
}


#[bench]
fn bench_html(b:&mut Bencher) {
    let s = get_input();
    let mut parser = Parser::new("english").unwrap().with_html(true);
    let mut ast = parser.tokenize(&s).unwrap();
    parser.detect_local(&mut ast, 2.0);
    b.iter(|| {
        parser.ast_to_html(&mut ast.clone(), false);
    });
}

#[bench]
fn bench_html2(b:&mut Bencher) {
    let s = get_input();
    let mut parser = Parser::new("english").unwrap().with_html(false);
    let mut ast = parser.tokenize(&s).unwrap();
    parser.detect_local(&mut ast, 2.0);
    b.iter(|| {
        parser.ast_to_html(&mut ast.clone(), false);
    });
}

#[bench]
fn bench_terminal(b:&mut Bencher) {
    let s = get_input();
    let mut parser = Parser::new("english").unwrap();
    let mut ast = parser.tokenize(&s).unwrap();
    parser.detect_local(&mut ast, 2.0);
    b.iter(|| {
        parser.ast_to_terminal(&ast.clone());
    });
}

#[bench]
fn bench_markdown(b:&mut Bencher) {
    let s = get_input();
    let mut parser = Parser::new("english").unwrap();
    let mut ast = parser.tokenize(&s).unwrap();
    parser.detect_local(&mut ast, 2.0);
    b.iter(|| {
        parser.ast_to_markdown(&mut ast.clone());
    });
}

#[bench]
fn bench_tokenize(b:&mut Bencher) {
    let s = get_input();
    let mut parser = Parser::new("english").unwrap();
    b.iter(|| {
        parser.tokenize(&s).unwrap();
    });
}

#[bench]
fn bench_local(b:&mut Bencher) {
    let s = get_input();
//    let s = include_str!("../../README.md");
    let mut parser = Parser::new("english").unwrap();
    let ast = parser.tokenize(&s).unwrap();    
    b.iter(|| {
        parser.detect_local(&mut ast.clone(), 1.9);
    });
}

#[bench]
fn bench_local_fuzzy(b:&mut Bencher) {
    let s = get_input();
    let mut parser = Parser::new("english").unwrap().with_fuzzy(Some(0.5));
    let ast = parser.tokenize(&s).unwrap();    
    b.iter(|| {
        parser.detect_local(&mut ast.clone(), 1.9);
    });
}

#[bench]
fn bench_total(b:&mut Bencher) {
    let s = get_input();
    b.iter(|| {
        let mut parser = Parser::new("english").unwrap().with_fuzzy(Some(0.5));
        let mut ast = parser.tokenize(&s).unwrap();    
        parser.detect_local(&mut ast, 1.9);
        parser.ast_to_html(&mut ast, true);
    });
}


#[bench]
fn bench_local_readme(b:&mut Bencher) {
    //    let s = get_input();
    let s = include_str!("../../README.md");
    let mut parser = Parser::new("english").unwrap();
    let ast = parser.tokenize(&s).unwrap();    
    b.iter(|| {
        parser.detect_local(&mut ast.clone(), 1.9);
    });
}

#[bench]
fn bench_local_readme_fuzzy(b:&mut Bencher) {
    //    let s = get_input();
    let s = include_str!("../../README.md");
    let mut parser = Parser::new("english").unwrap().with_fuzzy(Some(0.5));
    let ast = parser.tokenize(&s).unwrap();    
    b.iter(|| {
        parser.detect_local(&mut ast.clone(), 1.9);
    });
}

#[bench]
fn bench_global(b:&mut Bencher) {
    let s = get_input();
    let mut parser = Parser::new("english").unwrap();
    let ast = parser.tokenize(&s).unwrap();
    b.iter(|| {
        parser.detect_global(&mut ast.clone(), 0.01);
    });
}



