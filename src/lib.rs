// This file is part of Caribon.
//
// Caribon is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 of the License, or
// (at your option) any later version.
//
// Caribon is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Caribon.  If not, see <http://www.gnu.org/licenses/>.

/*! Repetition detector library

Detects the repetitions in an input file, using a stemming library in order to detect
words that are not technically identical but are quite the same, such as `cat` and `cats`.

Here's a short example (more details below):
```
use Caribon::Parser;
let parser = Parser::new("English").unwrap();
let words = parser.tokenize("Some text where you want to detect repetitions");
let mut repetitions = parser.detect_local(words, 1.5);
repetitions = parser.detect_global(words, 0.01);
let html = parser.words_to_html(&repetitions);
println!("{}", html);
```

Since this stemming is dependent on the language, you must first create a new `Parser` with the 
appropriate language. `Parser::new` returns a `caribon::Result<Parser>`, which will contain `Ok(Parser)` 
if the language is implemented, `Err(caribon::Error)` else: 

```
use caribon::Parser;
let result = Parser::new("foo");
assert!(result.is_err());
```

Once you have a parser, you can then configure it with various options:

```
use caribon::Parser;
let parser = Parser::new("english").unwrap()
                                            .with_html(true)
                                            .with_ignore_proper(true)
                                            .with_max_distance(20)
                                            .with_ignored("some, words, to, ignore");
```

The next step is to read some string and convert it to some inner format (currently `Vec<Word>`
but it is possible that it will change):

```ignore
let words = parser.tokenize(some_string).unwrap();
```

As `new`, this method can fail, so it returns a `Result`.

You then have a choice between multiple repetition detection algorithms. `detect_local` is probably
the one you want to use:

```ignore
let repetitions = parser.detect_local(words, 1.5);
```

There is also `detect_leak`, which is a particular algorithm which probably will be removed at some 
point:

```ignore
let repetitions = parser.detect_leak(words, 1.5);
```

The last one is `detect_global`, which detects repetiion in the whole file:

```ignore
let repetitions = parser.detect_global(words, 0.01);
```

Once you have detected those repetitions, the final step is to print them. 
`words_to_html` does this. Besides a reference to a <Vec<Word>>, it takes two arguments:
a threshold (basically, the number of repetitions to trigger underlining in HTML), and a 
boolean that tests whether the HTML code must be a standalone file or not (you will probably
want to set it to true).

```ignore
let html = parser.words_to_html(&repetitions, true):
```
*/

// Uncomment this if you use nightly and want to run benchmarks
#![feature(test)]
mod bench;
mod word;
mod parser;


pub use parser::Error;
pub use parser::Result;
pub use word::Word;
pub use parser::Parser;
