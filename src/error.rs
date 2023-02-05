//! Error types used by this library
use ::quick_xml;
use quick_xml::events::attributes::AttrError;
use std::fmt;
use std::num::{ParseFloatError, ParseIntError};
use std::str;

/// All the possible errors returned by this library
#[derive(Debug)]
pub enum Error {
    /// When parsing an angle and its not between 0 and 360 degrees
    AngleOutOfRange,
    /// When trying to parse a paper name and it doesn't make sense or it is not implemented yet
    UnknownPaper,
    ParseInt(ParseIntError),
    ParseFloat(ParseFloatError),
    ParseBool(str::ParseBoolError),
    IOError(std::io::Error),
    XMLError(quick_xml::Error),
    XMLAttrError(AttrError),
    /// Attempted to parse something that wasn't a valid xml document
    BadDocument,
    UTF8Error(str::Utf8Error),
    /// A problem trying to parse a hex colour, likely the value is too short
    ColourError,
}

impl From<ParseIntError> for Error {
    fn from(other: ParseIntError) -> Self {
        Error::ParseInt(other)
    }
}

impl From<ParseFloatError> for Error {
    fn from(other: ParseFloatError) -> Self {
        Error::ParseFloat(other)
    }
}

impl From<str::ParseBoolError> for Error {
    fn from(other: str::ParseBoolError) -> Self {
        Error::ParseBool(other)
    }
}

impl From<std::io::Error> for Error {
    fn from(other: std::io::Error) -> Self {
        Error::IOError(other)
    }
}

impl From<quick_xml::Error> for Error {
    fn from(other: quick_xml::Error) -> Self {
        Error::XMLError(other)
    }
}

impl From<AttrError> for Error {
    fn from(other: AttrError) -> Self {
        Error::XMLAttrError(other)
    }
}

impl From<str::Utf8Error> for Error {
    fn from(other: str::Utf8Error) -> Self {
        Error::UTF8Error(other)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", *self)
    }
}
