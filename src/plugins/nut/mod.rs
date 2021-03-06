pub mod catchers;
pub mod controllers;
pub mod models;
pub mod request;
pub mod tasks;

use std::fmt;
use std::str::FromStr;

use failure::Error as FailureError;

use super::super::errors::{Error, Result};

pub enum MediaType {
    TEXT,
    HTML,
    MARKDOWN,
}

impl fmt::Display for MediaType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MediaType::TEXT => write!(fmt, "text"),
            MediaType::HTML => write!(fmt, "html"),
            MediaType::MARKDOWN => write!(fmt, "markdown"),
        }
    }
}

impl FromStr for MediaType {
    type Err = FailureError;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "text" => Ok(MediaType::TEXT),
            "markdown" => Ok(MediaType::MARKDOWN),
            "html" => Ok(MediaType::HTML),
            t => Err(Error::BadMediaType(t.to_string()).into()),
        }
    }
}
