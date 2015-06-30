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

/// `Word` type: some inner representation used by `Parser`.
///
/// You probably should not use this type directly.
#[derive(Debug,Clone)]
pub enum Word {
    /// A String which is not part of the text (typically whitespace, HTML formatting, ...)
    Untracked(String),
    /// A word that is ignored, either because it is in `parser.ignored` or because it is
    /// a proper noun and `ignore_proper` has been set to `true`.
    Ignored(String),
    /// Tracked string, containing the string, the stemmed variant of the
    /// string, some value corresponding to the degree of repetitions and
    /// an option to a highlighting colour
    Tracked(String, String, f32, Option<&'static str>),
}

impl Word {
    /// Sets the repetition value of a word.
    pub fn set_count(&mut self, x: f32) 
    {
        match self {
            &mut Word::Tracked(_, _, ref mut v, _) => {
                *v = x;
            },
            _ => {}
        }
    }
}

