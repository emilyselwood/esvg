//! Error types used by this library
use ::quick_xml;
use quick_xml::events::attributes::AttrError;
use std::num::{ParseFloatError, ParseIntError};
use std::str;
use thiserror::Error;

/// All the possible errors returned by this library
#[derive(Error, Debug)]
pub enum Error {
    /// When parsing an angle and its not between 0 and 360 degrees
    #[error("Angle out of range: {0:?}")]
    AngleOutOfRange(f64),
    /// When trying to parse a paper name and it doesn't make sense or it is not implemented yet
    #[error("Unknown paper: {0:?}")]
    UnknownPaper(String),
    #[error("A style tag is badly formed")]
    MalformedStyle,
    #[error("Could not parse an integer: {0:?}")]
    ParseInt(ParseIntError),
    #[error("Could not parse a float: {0:?}")]
    ParseFloat(ParseFloatError),
    #[error("Could not parse a bool: {0:?}")]
    ParseBool(str::ParseBoolError),
    #[error("An io error: {0:?}")]
    IOError(std::io::Error),
    #[error("An XML error: {0:?}")]
    XMLError(quick_xml::Error),
    #[error("An XML attribute error: {0:?}")]
    XMLAttrError(AttrError),
    /// Attempted to parse something that wasn't a valid xml document
    #[error("Tried to parse document but it was empty")]
    EmptyDocument,
    #[error("A utf-8 encoding error: {0:?}")]
    UTF8Error(str::Utf8Error),
    /// A problem trying to parse a hex colour, likely the value is too short
    #[error("Invalid colour '{0:?}'")]
    ColourError(String),
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
