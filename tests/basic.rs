//! Basic integration tests of the public API of the crate

use envfig::{EnvVarDef, LoadError};

#[test]
fn load_test() {
    let env_var_name = "ENV_VAR";
    let env_var_value = 123;

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

#[test]
fn load_with_default_test() {
    let env_var_name = "ENV_VAR";
    let env_var_value = 123;
    let default = 124;

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
