//! Tests for `EnvVarDef` derive macro

use envfig::EnvVarDef;

#[proptest::property_test]
fn load_test(env_var_value: u8) {
    #[derive(EnvVarDef)]
    #[allow(dead_code)]
    struct TestEnvVars {
        env_var: u8,
    }
    let env_var_name = "ENV_VAR" ; // upper case `TestEnvVars::env_var` field name
    unsafe {
        std::env::set_var(env_var_name, env_var_value.to_string());
    }
    let env_vars = TestEnvVars::load().unwrap();
    assert_eq!(env_vars.env_var, env_var_value);

    unsafe {
        std::env::remove_var(env_var_name);
    }
    assert!(TestEnvVars::load().is_err());

    unsafe {
        std::env::set_var(env_var_name, "not a u8 type");
    }
    assert!(TestEnvVars::load().is_err());
}
