use std::fmt::Display;

use crate::EnvVarDef;

impl<T: Display> Display for EnvVarDef<T> {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        writeln!(f, "{} {}\n", console::Emoji::new("🔹", ""), self.name,)?;

        if self.title.is_some() || self.description.is_some() {
            write!(f, "{}", console::Emoji::new("📖", ""))?;
        }

        if let Some(title) = &self.title {
            writeln!(f, " {title}\n",)?;
        }

        if let Some(desc) = &self.description {
            writeln!(f, " {desc}\n",)?;
        }

        if let Some(default) = &self.default {
            writeln!(f, "{}  Default: {default}", console::Emoji::new("🛠️", ""),)?;
        }

        if let Some(example) = &self.example {
            writeln!(
                f,
                "{} Example: export {}={example}",
                console::Emoji::new("🧪", ""),
                self.name,
            )?;
        }
        Ok(())
    }
}

#[test]
fn doc_test() {
    let env_var = EnvVarDef::<u8>::new("SOME_ENV_VAR")
        .with_title("Some title")
        .with_description("Some huge description with \n making a new line")
        .with_default(10)
        .with_example(15);

    println!("{env_var}");

    println!("------");
    let env_var = EnvVarDef::<u8>::new("SOME_ENV_VAR")
        .with_description("Some huge description with \n making a new line")
        .with_default(10)
        .with_example(15);

    println!("{env_var}");

    println!("------");
    let env_var = EnvVarDef::<u8>::new("SOME_ENV_VAR")
        .with_default(10)
        .with_example(15);

    println!("{env_var}");
}
