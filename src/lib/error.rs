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

use std::error;
use std::result;
use std::fmt;

#[derive(Debug)]
/// Caribon error type (just a String currently)
pub struct Error {
    pub content: String,
}

impl Error {
    pub fn new(s: &str) -> Error {
        Error { content: s.to_owned() }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.content)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.content
    }
}

/// Caribon Result, used by some functions
pub type Result<T> = result::Result<T, Error>;
