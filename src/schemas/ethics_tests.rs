use super::ethics::{Path, ETHICA};
use std::str::FromStr;

#[test]
fn path_from_str_returns_err_on_invalid_path() {
    assert!(Path::from_str("meow").is_err());
    assert!(Path::from_str("pars:definitio").is_err());
    assert!(Path::from_str("pars(1):definitio(2)(3)").is_err());
}

#[test]
fn path_from_str_returns_a_path_when_valid() {
    assert!(Path::from_str("pars(1):yolo").is_ok());
    assert!(
        Path::from_str("pars(9):praefatio:axioma(3):aliter:scholium(2):demonstratio(3)").is_ok()
    );
}

#[test]
fn schema_contains_path_works() {
    assert!(ETHICA.contains_path(&"pars(1):propositio(1)".parse().unwrap()));
    assert!(!ETHICA
        .contains_path(&"pars(1):propositio(82)".parse().unwrap()));
}
