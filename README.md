Caribon
=======

A repetition detector written in Rust.

Why?
====

In some languages it is considered poor style to repeat a
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
`target/release`).

If you plan to use `cargo run`, note that command-line arguments must
be prefixed by `--` so cargo gives them to the binary: 

`$ cargo run -- --input=some_text.txt --output=output.html`

You can also install the `caribon` binary somewhere in your path
(e.g. `/usr/local/bin`) but currently there is no install/uninstall
option, so you'll have to do it manually.

Once you have generated an HTML file, just open it with your favorite
browser and see your repetitions. Words that are repeated too closely
are underlined in green, orange and red (depending on the number of
repetitions); words that appear globally too often are underlined in blue.

Examples
========

Here is an
[example](https://lady-segfault.github.io/caribon-examples/example_readme.html)
of Caribon used the HTML output of a (previous) version of this 
README, using the following command:

`cargo run -- --language=english --input=README.html
--output=example.html --fuzzy=0.5`

(Note that `--fuzzy=0.5`, while useful to show that fuzzy string
matching does indeed work, is not a very sensible parameter as is it
quite high (words only needs to be 50% similar to be considered the
same, matching `just` and `rust`); for real life usage, a lower value
would be recommended.)

Another [example](https://lady-segfault.github.io/caribon-examples/screenshot.png), displaying repetitions in
`README.md` to the terminal, using the following command:

`cargo run -- --language=english --input=README.md --fuzzy=0.5 | more`


Usage
=====

```
Caribon, version 0.5.2 by Élisabeth Henry <liz.henry@ouvaton.org>

Detects the repetitions in a text and highlights them

Options:
        --help: displays this message
        --version: displays program version
        --list-languages: lists the implemented languages
        --language=[language]: sets the language of the text (default: french)
        --input=[filename]: sets input file (default: stdin)
        --output=[filename]: sets output file (default: stdout)
        --ignore=[string]: a string containing custom ignored words, separated by spaces or comma
                (default: use a builtin list that depends on the language)
        --max-distance=[value]: sets max distance to be considered a repetition (in words) (default: 50)
        --threshold=[value]: sets threshold value for underlining local repetitions (default: 1.9)
        --global-threshold=[value|none]: activate global repetition detector and sets threshold value for underlining global repetitions
                (this threshold corresponds to the minimal ratio of words in the text, e.g. a threshold of 0.01 means
                that a word must represent at least 1% of the total words in the text to be underlined) (default: not activated)
        --input-format=[text|html]: sets input format (default: text, depends on input file extension)
        --output-format=[terminal|html]|markdown]: sets output format (default: terminal, depends on output file extension)
        --ignore-proper=[true|false]: if true, try to detect proper nouns and don't count them (default: false)
        --fuzzy=[value|none]: activate fuzzy string matching; value must be between 0.0 and 1.0 and corresponds to the maximal
                'difference' between two words until they are no more considered identical (e.g. 0.25 means that two words
                 must have no more than 25% of difference) (default: not activated)
```

Library
=======

It is possible to use Caribon as a library. The documentation is
available [here](http://lady-segfault.github.io/caribon/index.html); in order to
get the latest version, you can also generate it with
`cargo doc`.

Current features
================

* Built-in list of ignored words (common words whose repetitions don't
  matter) for french and english, though they are not complete.
* Stemming support for languages supported by the Snowball (http://snowball.tartarus.org/)
  project.
* Additionally (because stemming algorithms aren't always perfect, and sometimes
  you make typos), support for fuzzy string matching (based on Levenhstein distance).
* Count repetitions locally and globally.
* Detects HTML tags in input. It doesn't work with a full HTML file
  (containing `<html>`, `<body>` and so on) but it works fine if you
  use e.g `pandoc -o file.html file.md`.
* Outputs a basic HTML files which higlights the detected repetitions,
  or directly to the terminal, or to a Markdown file (with less useful information).

ChangeLog
=========

[See here](ChangeLog.md).

License
=======

Caribon is licensed under the [GNU General Public License](LICENSE), version 2.0
or (at your convenience) any ulterior version.

Credits
=======

Caribon is written by Élisabeth Henry <liz.henry at ouvaton.org>.

This software uses (rust bindings to) the
[C Stemming library](http://snowball.tartarus.org/dist/libstemmer_c.tgz)
written by Dr Martin Porter, licensed under the BSD License.

It also uses the [Rust implementation](https://crates.io/crates/edit-distance) of
Levenshtein distance written by Florian Ebelling, licensed under the Apache 2.0 License.

ToDo 
====

Library
-------
* Render prettier output files;
* Allow tokenizer to have in input "full" (with <html>,<head>,<body>
  tags) HTML documents;
* Complete builtin lists of ignored words and provide them for other
  languages (currently, only french, and english);
* Provide algorithm to detect repetitions of expressions, not just
  single words;
* Find better default values;
* Enhance documentation and add tests.

See also 
---------

[caribon-server](https://github.com/lady-segfault/caribon-server), a
work-in-progress project that runs Caribon as a web server.
