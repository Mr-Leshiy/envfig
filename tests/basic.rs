//! Basic integration tests of the public API of the crate

use envfig::{EnvVarDef, LoadError};

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
    assert!(matches!(
        env_var.clone().load(),
        Err(LoadError::CannotLoad(_, _))
    ));

    unsafe {
        std::env::set_var(env_var_name, "not a u8 type");
    }
    assert!(matches!(
        env_var.clone().load(),
        Err(LoadError::CannotParse(_, _, _))
    ));
}

#[proptest::property_test]
fn load_with_default_test(env_var_value: u8, default: u8) {
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
