# Envfig

A flexible and type-safe system for defining, documenting, and validating environment
variables.

The `EnvVarDef` type provides a builder-style interface to define environment
variables with optional default values, human-friendly metadata (title, description,
example), and custom validation logic.

It supports loading required or optional variables using `EnvVarDef::load` and
`EnvVarDef::load_option`, and will return rich error types when parsing or
validation fails.

# Example
```rust
use std::{env, str::FromStr};

use envfig::{EnvVarDef, LoadError, validator::Validator};

// A simple validator that ensures the value is greater than zero
struct Positive;

impl Validator<i32> for Positive {
    type Err = String;

    fn validate(
        self,
        val: i32,
    ) -> Result<i32, Self::Err> {
        if val > 0 {
            Ok(val)
        } else {
            Err("Value must be positive".into())
        }
    }
}

// Set an environment variable
unsafe {
    env::set_var("APP_PORT", "8080");
}

let port_env = EnvVarDef::new("APP_PORT")
    .with_title("Application Port")
    .with_description("The port the application listens on")
    .with_example(8080)
    .with_validator(Positive);

println!("doc: {}", port_env.doc());
let port = port_env.load().unwrap();
```

# Features
- Type-safe parsing from environment variables.
- Optional default value fallback.
- Full metadata support (title, description, example).
- Custom validation support via the `Validator` trait.
- Granular error types via `LoadError`.

# Traits
- `Validator<T>` is required to perform validation of environment variable values.
