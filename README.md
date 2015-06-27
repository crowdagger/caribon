Caribon
=======

A repetition detector written in Rust.

Why?
====

I don't think it's really the case in english, but in french (and
possibly other languages), it is considered poor style to repeat a
word too often in a text (particularly a literary text). The purpose
of this tool is to assist a writer in detecting those repetitions.

Why the name?
=============

A text is composed of words, themselves composed of characters, which
in french are called *caractères*. In french, *good* is *bon* so
*caribon* is essentially *good characters*.

Alright, this doesn't make much sense, I'll admit I just found the
name funny.

How?
====

Internally, Caribon use a stemming library
(https://github.com/lady-segfault/stemmer-rs, the Rust bindings for
Snowball C implementation: http://snowball.tartarus.org/) to reduce
words to their stems, which allows e.g. to see a singular and a plural
as the "same" word. Then it's just counting the repetitions, and
ouputting HTML.

Build
=====

You'll need Rust and Cargo: http://www.rust-lang.org/install.html,
then

`$ cargo build`

should do the job (it works with Rust 1.1). You can then run caribon either with:

`$ cargo run`

or by directly executing the binary (in `target/debug` or
`target/release`). Since, at this moment, all `caribon` does is read
a text on standard input and prints HTML on standard output, the
latter method is preferred:

`$ target/debug/caribon < some_text.txt > output.html`

(If you use `cargo run` instead, the standard output will start with
some message from Cargo, which will ruin the HTML formatting.)

You can also install the `caribon` binary somewhere in your path
(e.g. `/usr/local/bin`) but currently there is no install/uninstall
option.

Once you have generated an HTML file, just open it with your favorite
browser and see your repetitions. Note that the default binary is
configured for french, if you want to use another language, you'll
have (at this point) to modify `main.rs` by hand. Sorry.

Library
=======
It is possible to use Caribon as a library. There is currently no
documentation online, but you should be able to generate it with
`cargo doc`.

Basically, it's pretty easy:

You create a new parser with `Parser::new("language")` (the only
trick is that it returns an `Option`, as all languages are not
implemented, see `Parser::list_languages()` to get a vector of those
that are implemented by the stemming library.

You can then set some parameters for the parser, e.g:

```rust
let parser = Parser::new("french")
    .unwrap()
    .with_html(true) // enable html in input (default value, so it's useless)
    .with_ignore_proper(true); // don't count repetitions for proper nouns 
```

The first step is to "tokenize" the string you want to parse:

```rust
let words = parser.tokenize("Some string which may or may not contain repetitions");
```

The second step is to detect the repetitions, using one of the three
algorithms:

```rust
let detected_words = parser.detect_local(words);
let detected_words = parser.detect_global(words);
let detected_words = parser.detect_leak(words);
```

The final step is to display this vector of words. `Caribon` provides
a function that generates an HTML file:

```rust
println!("{}", caribon::words_to_html(detected_words));
```



Current features
================

* Basic support for french. Less than basic support for other
  languages supported by the snowall (http://snowball.tartarus.org/)
  project.
* Count repetitions either locally (either by ignoring repetitions
  after a given distance, or using some leak-based algorithm) or globally.
* Detects HTML tags in input. It doesn't work with a full HTML file
  (containing `<html>`, `<body>` and so on) but it works fine if you
  use e.g `pandoc -o file.html file.md`.
* Outputs a basic HTML files which higlights the detected repetitions.


TODO 
====

* Render prettier output files;
* Add arguments to `caribon` instead of having to modify `main.rs` and
recompile;
* Make a tiny webserver version so it can be used online?
