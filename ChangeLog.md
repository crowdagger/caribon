ChangeLog
=========

0.6.0 (???)
-----------
* Added `Parser.with_more_ignored` to add ignored words instead of replacing them
* Added `--add-ignored=` option for the binary
* Caribon now supports working on full HTML files, not just HTML
  fragments. This necessitated some changes in the API:
   * Use a struct `Ast` instead of `Vec<Word>`
   * Renamed `words_to_*` to `ast_to_*`
   * `detect_[local|global]` methods now takes a mutable ref to an Ast
   instead of moving a `Vec<Word>` and returning a new one
* Removed `detect_leak` algorithm
* Added special "no_stemmer" option to `Parser::new`: disables
stemming
* Fixed a bug in fuzzy string matching



0.5.2 (2015-07-03)
------------------
* Added `Parser.words_to_terminal` that outputs repetitions for the
terminal instead of using HTML, using terminal colours characters
* Added `Parser.words_to_markdown` that outputs repetitions in
markdown
* Added fuzzy string matching possibility, enabled by constructing the
parser with the `with_fuzzy` method
* Reworked options for the binary:
  * Added `--fuzzy=` to trigger fuzzy string matching
  * Disabled global detection by default
  * Added `--input-format` and `--output-format` options
  * Without these options, auto-detect input and output formats based
    on file extensions

0.5.1 (2015-07-01)
------------------
* Bugfixes in `Parser.tokenize` and `Parser.words_to_html` when
  handling non-html input text

0.5.0 (2015-07-01)
------------------
* Improved documentation
* `Parser.tokenize` now returns a `Result`, giving an `Error` instead of
panicking
* `Parser::new` also returns a `Result` instead of an `Option`
* `detect_leak` now uses `max_distance` to compute a leak, so `leak`
parameter is removed.
* Therefore, `--leak=x` argument is removed from the binary
* Options for the binary have some more changes (see `--help`)
* It is now possible to chain detection methods, highlighting
with different colours
* Added `words_stats` method



0.4.0 (2015-06-30)
------------------
* `words_to_html` is now part of Parser implementation
* If `parser.html` is false (input type is not html),
  `parser.words_to_html` convert line breaks to `<br/>`
* Escape characters such as `&nbsp;` are leaved as such if
  `parser.html` is true
* Better support for user-defined list of ignored words in the library
* New option `--ignore=` in the binary
  

0.3.0 (2015-06-29)
------------------
* `words_to_html` takes another argument, `standalone`
* removed "--debug" option
* added "--input" and "--output" options

0.2.2 (2015-06-28)
------------------
* Fixed a bug (and unnecessary clone) in detect_local algorithm
* Added a list of ignored words for english

0.2.1 (2015-06-28)
------------------
* Added documentation for the library and examples on Github.io
* Minor bugfixes

0.2.0 (2015-06-28)
------------------
* Added support for HTML in input
* Added some ignored words for french
* Added option to ignore proper nouns
* Added `detect_local` algorithm
* Added `Parser::list_languages()` function
* Added support for command-line arguments

0.1.0 (2015-06-27)
------------------

* first release
