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

/// `Word` type.
#[derive(Debug,Clone)]
pub enum Word {
    /// String which is not part of the text (typically whitepsaced, HTML formatting, ...)
    Untracked(String),
    /// Ignored word
    Ignored(String),
    /// Tracked string, containing the string, the stemmed variant of the
    /// string, and the degree of repetitions
    Tracked(String, String, f32)
}

impl Word {
    pub fn set_count(&mut self, x: f32) 
    {
        match self {
            &mut Word::Tracked(_, _, ref mut v) => {
                *v = x;
            },
            _ => {}
        }
    }
}
