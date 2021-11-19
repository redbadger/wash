use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::{convert::AsRef, str::FromStr};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum IdParseError {
    #[error(r#"found the prefix "{found}", but expected "{expected}""#)]
    WrongKeyType { found: String, expected: String },
    #[error("found length {0}, but should be 56 chars")]
    WrongLength(usize),
}

pub type ModuleId = Id<'M'>;
pub type ServerId = Id<'N'>;
pub type ServiceId = Id<'V'>;
pub type ClusterSeed = Seed<'C'>;

#[derive(Clone, Debug, Display, Eq, PartialEq, Serialize, Deserialize)]
pub struct Id<const PREFIX: char>(String);

impl<const PREFIX: char> FromStr for Id<PREFIX> {
    type Err = IdParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(parse(s, PREFIX, false)?))
    }
}

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
        Ok(Self(parse(s, PREFIX, true)?))
    }
}

fn parse(value: &str, prefix: char, is_seed: bool) -> Result<String, IdParseError> {
    let count = value.chars().count();
    if count != 56 {
        return Err(IdParseError::WrongLength(count));
    }

    let prefix = if is_seed {
        format!("S{}", prefix)
    } else {
        prefix.to_string()
    };

    if value.starts_with(&prefix) {
        Ok(value.to_string())
    } else {
        Err(IdParseError::WrongKeyType {
            found: value.chars().take(prefix.chars().count()).collect(),
            expected: prefix,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(
		"SC000000000000000000000000000000000000000000000000000000", 'C', true
		=> Ok("SC000000000000000000000000000000000000000000000000000000".to_string());
		"valid cluster seed")]
    #[test_case(
		"SC000000000000000000000000000000000000000000000000", 'C', true
		=> Err(IdParseError::WrongLength(50));
		"short cluster seed")]
    #[test_case(
		"SM000000000000000000000000000000000000000000000000000000", 'C', true
		=> Err(IdParseError::WrongKeyType{expected: "SC".to_string(), found: "SM".to_string()});
		"cluster seed has wrong prefix")]
    #[test_case(
		"M0000000000000000000000000000000000000000000000000000000", 'M', false
		=> Ok("M0000000000000000000000000000000000000000000000000000000".to_string());
		"valid module id")]
    #[test_case(
		"M0000000000000000000000000000000000000000000000000", 'M', false
		=> Err(IdParseError::WrongLength(50));
		"short module id")]
    #[test_case(
		"V0000000000000000000000000000000000000000000000000000000", 'M', false
		=> Err(IdParseError::WrongKeyType{expected: "M".to_string(), found: "V".to_string()});
		"module id has wrong prefix")]
    fn test_parse(value: &str, prefix: char, is_seed: bool) -> Result<String, IdParseError> {
        parse(value, prefix, is_seed)
    }
}
