//! Basic integration tests of the public API of the crate

use envfig::EnvVarDef;

#[test]
fn load_test() {
    let env_var_name = "ENV_VAR".to_string();
    let env_var_value = 123;

    unsafe {
        std::env::set_var(&env_var_name, env_var_value.to_string());
    }
    let env_var: EnvVarDef<u8> = EnvVarDef::new(env_var_name.clone());
    assert_eq!(env_var.clone().load().unwrap(), env_var_value);

    unsafe {
        std::env::remove_var(&env_var_name);
    }
    assert!(env_var.clone().load().is_err());

    unsafe {
        std::env::set_var(&env_var_name, "not a u8 type");
    }
    assert!(env_var.clone().load().is_err());
}
