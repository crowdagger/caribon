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

// Code to end shell colouring
pub const SHELL_COLOUR_OFF: &'static str = "\x1B[0m";

// Javascript function to print repetitions
pub const SCRIPTS: &'static str = include_str!("html/scripts.js");

/// Get a shell colour from a string
pub fn get_shell_colour(colour: &str) -> Option<&'static str> {
    match colour {
        "red" => Some("\x1B[4;31m"),
        "green" => Some("\x1B[4;32m"),
        "cyan" => Some("\x1B[4;36m"),
        "brown" => Some("\x1B[4;33m"),
        "blue" => Some("\x1B[4;32m"),
        "purple" => Some("\x1B[4;35m"),
        "orange" => Some("\x1B[4;33m"),
        _ => None,
    }
}

/// Generate the style attribute according to x and threshold
pub fn value_to_colour(x: f32, threshold: f32) -> &'static str {
    if x < threshold {
        panic!("value_to_colour called with x < threshold");
    } else if x < 1.5 * threshold {
        "green"
    } else if x < 2.0 * threshold {
        "orange"
    } else {
        "red"
    }
}
