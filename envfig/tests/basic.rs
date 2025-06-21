//! Basic integration tests of the public API of the crate

use std::fmt::Debug;

use envfig::{EnvVarDef, validator::Validator};

#[proptest::property_test]
fn load_test(env_var_value: u8) {
    let env_var_name = "ENV_VAR";
    unsafe {
        std::env::set_var(env_var_name, env_var_value.to_string());
    }
    let env_var: EnvVarDef<u8> = EnvVarDef::new(env_var_name);
    assert_eq!(env_var.clone().load().unwrap(), env_var_value);

    unsafe {
        std::env::remove_var(env_var_name);
    }
    assert!(env_var.clone().load().is_err(),);

    unsafe {
        std::env::set_var(env_var_name, "not a u8 type");
    }
    assert!(env_var.clone().load().is_err(),);
}

#[proptest::property_test]
fn load_with_default_test(
    env_var_value: u8,
    default: u8,
) {
    let env_var_name = "ENV_VAR";

    unsafe {
        std::env::set_var(env_var_name, env_var_value.to_string());
    }
    let env_var: EnvVarDef<u8> = EnvVarDef::new(env_var_name).with_default(default);
    assert_eq!(env_var.clone().load().unwrap(), env_var_value);

    unsafe {
        std::env::remove_var(env_var_name);
    }
    assert_eq!(env_var.clone().load().unwrap(), default);

    unsafe {
        std::env::set_var(env_var_name, "not a u8 type");
    }
    assert_eq!(env_var.clone().load().unwrap(), default);
}

#[derive(Clone)]
struct TestValidator<T> {
    input: T,
    output: Result<T, ()>,
}
impl<T: PartialEq + Debug> Validator<T> for TestValidator<T> {
    fn validate(
        self,
        val: T,
    ) -> anyhow::Result<T> {
        assert_eq!(val, self.input);
        self.output.map_err(|()| anyhow::anyhow!("Some error"))
    }
}

#[proptest::property_test]
fn load_with_validator_test(
    env_var_value: u8,
    ret_value: u8,
) {
    let env_var_name = "ENV_VAR";

    unsafe {
        std::env::set_var(env_var_name, env_var_value.to_string());
    }

    let env_var: EnvVarDef<u8, _> = EnvVarDef::new(env_var_name).with_validator(TestValidator {
        input: env_var_value,
        output: Ok(ret_value),
    });
    assert_eq!(env_var.clone().load().unwrap(), ret_value);

    let env_var: EnvVarDef<u8, _> = EnvVarDef::new(env_var_name).with_validator(TestValidator {
        input: env_var_value,
        output: Err(()),
    });
    assert!(env_var.clone().load().is_err());
}

#[proptest::property_test]
fn load_option_test(env_var_value: u8) {
    let env_var_name = "ENV_VAR";

    unsafe {
        std::env::set_var(env_var_name, env_var_value.to_string());
    }
    let env_var: EnvVarDef<u8> = EnvVarDef::new(env_var_name);
    assert_eq!(env_var.clone().load_option().unwrap(), Some(env_var_value));

    unsafe {
        std::env::remove_var(env_var_name);
    }
    assert_eq!(env_var.clone().load_option().unwrap(), None);

    unsafe {
        std::env::set_var(env_var_name, "not a u8 type");
    }
    assert_eq!(env_var.clone().load_option().unwrap(), None);
}

#[proptest::property_test]
fn load_option_with_validator_test(
    env_var_value: u8,
    ret_value: Option<u8>,
) {
    let env_var_name = "ENV_VAR";

    unsafe {
        std::env::set_var(env_var_name, env_var_value.to_string());
    }
    let env_var: EnvVarDef<u8, _> = EnvVarDef::new(env_var_name).with_validator(TestValidator {
        input: Some(env_var_value),
        output: Ok(ret_value),
    });
    assert_eq!(env_var.clone().load_option().unwrap(), ret_value);

    let env_var: EnvVarDef<u8, _> = EnvVarDef::new(env_var_name).with_validator(TestValidator {
        input: Some(env_var_value),
        output: Err(()),
    });
    assert!(env_var.clone().load_option().is_err());

    let env_var: EnvVarDef<u8, _> = EnvVarDef::new(env_var_name).with_validator(TestValidator {
        input: None,
        output: Ok(ret_value),
    });
    unsafe {
        std::env::remove_var(env_var_name);
    }
    assert_eq!(env_var.clone().load_option().unwrap(), ret_value);

    unsafe {
        std::env::set_var(env_var_name, "not a u8 type");
    }
    assert_eq!(
        env_var
            .clone()
            .with_validator(TestValidator {
                input: None,
                output: Ok(ret_value),
            })
            .load_option()
            .unwrap(),
        ret_value
    );
}
