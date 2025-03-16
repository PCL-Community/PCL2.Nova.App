pub mod base;
pub mod launch;
pub mod download;

use std::str::FromStr;

use anyhow::Error;

#[derive(Debug)]
pub struct NovaMCoreError {
    msg: String,
}

impl std::fmt::Display for NovaMCoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[PCL2 Nova Minecraft Core] [ERROR] {}", self.msg).expect("Faild to format content.");
        Ok(())
    }
}

impl std::error::Error for NovaMCoreError {}

impl FromStr for NovaMCoreError {
    type Err = Self;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { msg: s.to_string() })
    }
}

impl From<Error> for NovaMCoreError {
    fn from(value: Error) -> Self {
        Self {
            msg: value.to_string(),
        }
    }
}

impl NovaMCoreError {
    pub fn msg<S>(msg: &S) -> Self
    where
        S: ToString + ?Sized,
    {
        Self {
            msg: msg.to_string(),
        }
    }
}
