use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::{convert::AsRef, str::FromStr};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IdParseError {
    #[error(r#"found the prefix "{found}", but expected "{expected}""#)]
    WrongKeyType { found: char, expected: char },
    #[error("found length {0}, but should be 56 chars")]
    WrongLength(usize),
}

#[derive(Clone, Debug, Display, Eq, PartialEq, Serialize, Deserialize)]
pub struct Id<const PREFIX: char>(String);

impl<const PREFIX: char> FromStr for Id<PREFIX> {
    type Err = IdParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let count = s.chars().count();
        if count != 56 {
            return Err(IdParseError::WrongLength(count));
        }

        if s.starts_with(PREFIX) {
            Ok(Self(s.to_string()))
        } else {
            let found = s
                .chars()
                .next()
                .expect("we already know it's the right length");
            Err(IdParseError::WrongKeyType {
                found,
                expected: PREFIX,
            })
        }
    }
}

pub type ModuleId = Id<'M'>;
pub type ServerId = Id<'N'>;
pub type ServiceId = Id<'V'>;

#[derive(Clone, Debug, Display, Eq, PartialEq, Serialize, Deserialize)]
pub struct Seed<const PREFIX: char>(String);

// TODO: We probably need to understand what this should mean :-)
impl<const PREFIX: char> Default for Seed<PREFIX> {
    fn default() -> Self {
        Self("SC000000000000000000000000000000000000000000000000000000".to_string())
    }
}

impl<const PREFIX: char> AsRef<str> for Seed<PREFIX> {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl<const PREFIX: char> FromStr for Seed<PREFIX> {
    type Err = IdParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let count = s.chars().count();
        if count != 56 {
            return Err(IdParseError::WrongLength(count));
        }

        if s.starts_with(&format!("S{}", PREFIX)) {
            Ok(Self(s.to_string()))
        } else {
            let found = s
                .chars()
                .nth(1)
                .expect("we already know it's the right length");
            Err(IdParseError::WrongKeyType {
                found,
                expected: PREFIX,
            })
        }
    }
}

pub type ClusterSeed = Seed<'C'>;
