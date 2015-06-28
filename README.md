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
outputting HTML.

Build
=====

You'll need Rust and Cargo, see [install instructions](http://www.rust-lang.org/install.html). Then

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
option, so you'll have to do it manually.

Once you have generated an HTML file, just open it with your favorite
browser and see your repetitions. Note that the default binary is
configured for french, if you want to use another language, you'll
have to pass an option (see below). Note that though a variety of input
languages are supported thanks to the Snowball stemming library, at
this time only french has a (incomplete) list of common words to
ignore. 

Usage
=====
```
Reads a file on stdint and outputs an HTML file showing the repetitions

Options:
--help: displays this message
--version: displays program version
--list_languages: lists the implemented languages
--language=[language]: sets the language of the text (default: french)
--algo=[global|local|leak]: sets the detection algoritm (default: local)
--leak=[value]: sets leak value (only used by leak algorithm) (default: 0.95)
--max_distance=[value]: sets max distance (only used by local algorithm) (default: 50)
--global_count=[relative|absolute]: sets repetitions count as absolute or relative ratio of words
                       (only used by global algorithm) (default: absolute)
--threshold=[value]: sets threshold value for underlining repetitions (default: 1.9)
--html=[true|false]: enables/disable HTML input (default: true)
--ignore_proper=[true|false]: if true, try to detect proper nouns and don't count them (default: false)
--debug=[true|false]: if true, print the internal data instead of HTML (default: false)
```

Examples
========

Here is an [example](https://lady-segfault.github.io/caribon-examples/example_readme.html) of Caribon used on a (previous) version of this
README, using the following command:

`target/debug/caribon --language=english --algo=local
--max_distance=25 --threshold=2.0 --ignore_proper=true < README.html  > example.html`

As you can see, it is currently not very suited for english. Here is
an [example in french](https://lady-segfault.github.io/caribon-examples/example_readme.fr.html), using a translation of (the beginning of) this
same file, using nearly the same command:

`target/debug/caribon --language=french --algo=local
--max_distance=25 --threshold=2.0 --ignore_proper=true < README.fr.html  > example.html`

Library
=======
It is possible to use Caribon as a library. The documentation is
available [here](http://lady-segfault.github.io/caribon/); in order to
get the latest version, you can also generate it with
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
let detected_words = parser.detect_global(words, false); 
let detected_words = parser.detect_leak(words);
```

The final step is to display this vector of words. `Caribon` provides
a function that generates an HTML file, which also takes as argument a
threshold above which words are underlined:

```rust
println!("{}", caribon::words_to_html(&detected_words, 1.5));
```

(A note on this threshold: its choices depends on the detection
algorithm you use (and possibly your taste and the language you write
in, of course). Generally, it should be a bit above 1.0, except for
`detect_global` (in which case, it depends whether you set
`is_relative` to true or false).



Current features
================

* Basic support for french. Less than basic support for other
  languages supported by the Snowball (http://snowball.tartarus.org/)
  project.
* Count repetitions either locally (either by ignoring repetitions
  after a given distance, or using some leak-based algorithm) or globally.
* Detects HTML tags in input. It doesn't work with a full HTML file
  (containing `<html>`, `<body>` and so on) but it works fine if you
  use e.g `pandoc -o file.html file.md`.
* Outputs a basic HTML files which higlights the detected repetitions.

ChangeLog
=========

[See here](ChangeLog.md).

License
=======

Caribon is licensed under the [GNU General Public License](LICENSE), version 2.0
or (at your convenience) any ulterior version.

Author
======

Élisabeth Henry <liz.henry at ouvaton.org>.

This software uses (rust bindings to) the
[C Stemming library](http://snowball.tartarus.org/dist/libstemmer_c.tgz)
written by Dr Martin Porter, licensed under the BSD License. 

TODO 
====

Library
-------
* Render prettier output files;
* Enhance the way language-dependent list of ignored words are
  treated, and provide them for other languages (currently, only
  french, and it should be completed);
* Provide algorithm to detect repetitions of expressions, not just
  single words;
* Enhance documentation and add tests.

Program
-------
* Add possibility to read and print to a file instead of stdin/stdout;
* Make a tiny webserver version so it can be used online? (Maybe more
  appropriate in another project)
