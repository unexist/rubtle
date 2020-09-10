use std::error::Error as StdError;
///
/// @package Rubtle-Lib
///
/// @file Error functions
/// @copyright 2020 Christoph Kappel <unexist@subforge.org>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
//
use std::fmt;

#[derive(Debug)]
pub struct Error {
    pub details: String,
}

impl Error {
    #[allow(dead_code)]
    fn new(details: &str) -> Error {
        Error {
            details: details.to_string(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        &self.details
    }
}
