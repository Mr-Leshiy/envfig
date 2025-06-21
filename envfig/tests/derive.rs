//! Tests for `EnvVarDef` derive macro

use envfig::EnvVarDef;

#[derive(EnvVarDef)]
#[allow(dead_code)]
struct TestEnvVars {}

#[test]
fn something() {
    TestEnvVars::load().unwrap();
}
