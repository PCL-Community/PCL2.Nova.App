use std::str::FromStr;

use anyhow::Error;

pub mod minecraft;

#[derive(Debug)]
pub struct NovaError {
    msg: String,
}

impl std::fmt::Display for NovaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[PCL2 Nova] [ERROR] {}", self.msg).expect("Faild to format content.");
        Ok(())
    }
}

impl std::error::Error for NovaError {}

impl FromStr for NovaError {
    type Err = Self;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { msg: s.to_string() })
    }
}

impl From<Error> for NovaError {
    fn from(value: Error) -> Self {
        Self {
            msg: value.to_string(),
        }
    }
}

impl NovaError {
    pub fn msg<S>(msg: &S) -> Self
    where
        S: ToString + ?Sized,
    {
        Self {
            msg: msg.to_string(),
        }
    }
}
