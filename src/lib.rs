#![allow(missing_docs, clippy::missing_docs_in_private_items, dead_code)]

use std::{env, fmt::Debug, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnvVarDef<T> {
    name: String,

    title: Option<String>,
    description: Option<String>,
    default: Option<T>,
    example: Option<T>,
}

impl<T> EnvVarDef<T> {
    pub fn new(name: String) -> Self {
        Self {
            name,
            title: None,
            description: None,
            default: None,
            example: None,
        }
    }
}

impl<T: FromStr> EnvVarDef<T>
where T: FromStr<Err: Debug>
{
    pub fn load(self) -> anyhow::Result<T> {
        let Ok(val) = env::var(&self.name) else {
            if let Some(default) = self.default {
                return Ok(default);
            }
            anyhow::bail!(
                "Cannot load Env Var {}, either not set or not valid unicode encoded.",
                self.name
            );
        };
        match val.parse() {
            Ok(res) => Ok(res),
            Err(err) => {
                if let Some(default) = self.default {
                    Ok(default)
                } else {
                    anyhow::bail!(
                        "Cannot parse Env Var {} value {val}, err: {err:?}",
                        self.name
                    );
                }
            },
        }
    }
}
