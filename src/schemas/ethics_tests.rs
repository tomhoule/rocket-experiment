#![cfg(test)]

use super::ethics::{Path, ETHICA};
use std::str::FromStr;
use test::Bencher;
use std::collections::*;
use inlinable_string::InlinableString;

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
        Path::from_str("pars(9):praefatio:axioma(3):aliter:scholium(2):dem(3)").is_ok()
    );
}

#[test]
fn schema_contains_path_works() {
    assert!(ETHICA.contains_path(&"pars(1):p(1)".parse().unwrap()));
    assert!(!ETHICA
        .contains_path(&"pars(1):p(82)".parse().unwrap()));
}

#[test]
fn expanded_paths_are_contained_paths() {
    let expanded = ETHICA.expand();
    assert!(expanded.len() > 900);

    for node in expanded {
        let path = Path::from_str(&node.path);
        assert!(path.is_ok(), "Path is ok: {:?}", node.path);
        let path = path.unwrap();
        assert!(ETHICA.contains_path(&path), "Path is contained: {:?}", path);
    }
}

#[bench]
fn bench_expand(bench: &mut Bencher) {
    bench.iter(|| ETHICA.expand());
}

#[bench]
fn bench_contains_path_whole_schema(bench: &mut Bencher) {
    let expanded = ETHICA.expand();
    bench.iter(|| {
        for node in &expanded {
            let path = Path::from_str(&node.path).unwrap();
            if !ETHICA.contains_path(&path) { return false }
        }
        true
    });
}

#[bench]
fn bench_contains_path_whole_schema_hashset(bench: &mut Bencher) {
    let expanded = ETHICA.expand();
    let set: HashSet<InlinableString> = expanded.iter().map(|node| node.path.clone()).collect();
    bench.iter(|| {
        for node in &expanded {
            assert!(set.contains(&node.path));
        }
    });
}

#[bench]
fn bench_contains_path_whole_schema_btreeset(bench: &mut Bencher) {
    let expanded = ETHICA.expand();
    let set: BTreeSet<InlinableString> = expanded.iter().map(|node| node.path.clone()).collect();
    bench.iter(|| {
        for node in &expanded {
            assert!(set.contains(&node.path));
        }
    });
}

#[bench]
fn bench_contains_path(bench: &mut Bencher) {
    bench.iter(|| assert!(ETHICA.contains_path(&"pars(3):p(25):dem".parse().unwrap())));
}
