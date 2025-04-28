#![allow(missing_docs, clippy::missing_docs_in_private_items, dead_code)]

mod doc;

use std::{env, fmt::Debug, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnvVarDef<T> {
    name: String,

    default: Option<T>,

    title: Option<String>,
    description: Option<String>,
    example: Option<T>,
}

impl<T> EnvVarDef<T> {
    pub fn new(name: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            title: None,
            description: None,
            default: None,
            example: None,
        }
    }

    pub fn with_default(
        mut self,
        default: T,
    ) -> Self {
        self.default = Some(default);
        self
    }

    pub fn with_title(
        mut self,
        title: impl ToString,
    ) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn with_description(
        mut self,
        description: impl ToString,
    ) -> Self {
        self.description = Some(description.to_string());
        self
    }

    pub fn with_example(
        mut self,
        example: T,
    ) -> Self {
        self.example = Some(example);
        self
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
