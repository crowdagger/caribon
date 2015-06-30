ChangeLog
=========

0.4.1 (???)
-----------
* Completed documentation

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
