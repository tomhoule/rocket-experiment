#![cfg(test)]

use super::ethics::{Path, ETHICS};
use std::str::FromStr;
use test::Bencher;
use std::collections::*;
use inlinable_string::{InlinableString, INLINE_STRING_CAPACITY};

#[test]
fn path_part() {
    assert_eq!(Path::from_str("pars/1:p/3").unwrap().part(), Some(1));
    assert_eq!(Path::from_str("pars/2:p/3").unwrap().part(), Some(2));
    assert_eq!(Path::from_str("pars/3:p/3:demo").unwrap().part(), Some(3));
}

#[test]
fn path_from_str_returns_err_on_invalid_path() {
    assert!(Path::from_str("meow").is_err());
    assert!(Path::from_str("pars:def").is_err());
    assert!(Path::from_str("pars/1:def/2/3").is_err());
}

#[test]
fn path_from_str_returns_a_path_when_valid() {
    assert!(Path::from_str("pars/1:yolo").is_ok());
    assert!(Path::from_str("pars/9:praefatio:axioma/3:aliter:scholium/2:dem/3").is_ok());
}

#[test]
fn schema_contains_path_works() {
    assert!(ETHICS.contains_path(&"pars/1:p/1".parse().unwrap()));
    assert!(!ETHICS.contains_path(&"pars/1:p/82".parse().unwrap()));
}

#[test]
fn expanded_paths_are_contained_paths() {
    let expanded = ETHICS.expand();
    assert!(expanded.len() > 900);

    for node in expanded {
        let path = Path::from_str(&node.path);
        assert!(path.is_ok(), "Path is ok: {:?}", node.path);
        let path = path.unwrap();
        assert!(ETHICS.contains_path(&path), "Path is contained: {:?}", path);
    }
}

#[test]
fn all_paths_are_inlinable() {
    let expanded = ETHICS.expand();
    for node in expanded {
        assert!(node.path.len() < INLINE_STRING_CAPACITY, "{}", node.path);
    }
}

#[bench]
fn bench_expand(bench: &mut Bencher) {
    bench.iter(|| ETHICS.expand());
}

#[bench]
fn bench_contains_path_whole_schema(bench: &mut Bencher) {
    let expanded = ETHICS.expand();
    bench.iter(|| {
        for node in &expanded {
            let path = Path::from_str(&node.path).unwrap();
            if !ETHICS.contains_path(&path) {
                return false;
            }
        }
        true
    });
}

#[bench]
fn bench_contains_path_whole_schema_hashset(bench: &mut Bencher) {
    let expanded = ETHICS.expand();
    let set: HashSet<InlinableString> = expanded.iter().map(|node| node.path.clone()).collect();
    bench.iter(|| {
        for node in &expanded {
            assert!(set.contains(&node.path));
        }
    });
}

#[bench]
fn bench_contains_path_whole_schema_btreeset(bench: &mut Bencher) {
    let expanded = ETHICS.expand();
    let set: BTreeSet<InlinableString> = expanded.iter().map(|node| node.path.clone()).collect();
    bench.iter(|| {
        for node in &expanded {
            assert!(set.contains(&node.path));
        }
    });
}

#[bench]
fn bench_contains_path(bench: &mut Bencher) {
    bench.iter(|| {
        assert!(ETHICS.contains_path(&"pars/3:p/25:dem".parse().unwrap()))
    });
}

#[bench]
fn bench_serialize_expanded(bench: &mut Bencher) {
    let expanded = ETHICS.expand();
    bench.iter(|| json!({ "expanded": expanded }));
}
