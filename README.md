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

Current features
================

* Basic support for french. Less than basic support for other
  languages supported by the snowall (http://snowball.tartarus.org/)
  project.
* Count repetitions either locally (using some leak-based algorithm)
  or globally.
* Outputs a basic HTML files which higlights the detected repetitions.

TODO 
====

* Add another local detector, based on distance;
* Add support for HTML in input files;
* Render prettier output files;
* Add arguments to `caribon` instead of having to modify `main.rs` and
recompile;
* Make a tiny webserver version so it can be used online.
