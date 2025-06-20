use std::fmt::{Display, Write};

use crate::{EnvVarDef, validator::Validator};

impl<T: Display, V: Validator<T>> EnvVarDef<T, V> {
    /// A human readable documentation `EnvVarDef` based on the  `description`, `title` on
    /// other fields. Which provides a comprehensive description of the `EnvVarDef`
    /// intance.
    pub fn doc(&self) -> String {
        let mut res = String::new();
        let _ = writeln!(
            &mut res,
            "{} {}\n",
            console::Emoji::new("🔹", ""),
            self.name,
        );

        if self.title.is_some() || self.description.is_some() {
            let _ = write!(&mut res, "{}", console::Emoji::new("📖", ""));
        }

        if let Some(title) = &self.title {
            let _ = writeln!(&mut res, " {title}\n",);
        }

        if let Some(desc) = &self.description {
            let _ = writeln!(&mut res, " {desc}\n",);
        }

        if let Some(default) = &self.default {
            let _ = writeln!(
                &mut res,
                "{}  Default: {default}",
                console::Emoji::new("🛠️", ""),
            );
        }

        if let Some(example) = &self.example {
            let _ = writeln!(
                &mut res,
                "{} Example: export {}={example}",
                console::Emoji::new("🧪", ""),
                self.name,
            );
        }

        if let Some(valiator) = &self.validator {
            if let Some(val_description) = valiator.description() {
                let _ = writeln!(
                    &mut res,
                    "{} {}",
                    console::Emoji::new("✅", ""),
                    val_description,
                );
            }
        }

        res
    }
}

#[test]
fn doc_test() {
    let env_var = EnvVarDef::<u8>::new("SOME_ENV_VAR")
        .with_title("Some title")
        .with_description("Some huge description with \n making a new line")
        .with_default(10)
        .with_example(15);

    println!("{}", env_var.doc());

    println!("------");
    let env_var = EnvVarDef::<u8>::new("SOME_ENV_VAR")
        .with_description("Some huge description with \n making a new line")
        .with_default(10)
        .with_example(15);

    println!("{}", env_var.doc());

    println!("------");
    let env_var = EnvVarDef::<u8>::new("SOME_ENV_VAR")
        .with_default(10)
        .with_example(15);

    println!("{}", env_var.doc());
}
