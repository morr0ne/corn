use corn::from_str;
use serde::Deserialize;
use std::borrow::Cow;
use std::fs;

// Borrowed structure for basic string test
#[derive(Deserialize, Debug, PartialEq)]
struct BorrowedBasic<'a> {
    #[serde(borrow)]
    foo: Cow<'a, str>,
}

// Borrowed structure for string test with multiple fields
#[derive(Deserialize, Debug, PartialEq)]
struct BorrowedString<'a> {
    #[serde(borrow)]
    foo: Cow<'a, str>,
    #[serde(borrow)]
    bar: Cow<'a, str>,
    #[serde(borrow)]
    baz: Cow<'a, str>,
    #[serde(borrow)]
    qux: Cow<'a, str>,
}

// Borrowed structure using Cow for flexible ownership
#[derive(Deserialize, Debug, PartialEq)]
struct BorrowedStringCow<'a> {
    #[serde(borrow)]
    foo: Cow<'a, str>,
    #[serde(borrow)]
    bar: Cow<'a, str>,
    #[serde(borrow)]
    baz: Cow<'a, str>,
    #[serde(borrow)]
    qux: Cow<'a, str>,
}

// Borrowed nested structure
#[derive(Deserialize, Debug, PartialEq)]
struct BorrowedNested<'a> {
    #[serde(borrow)]
    name: BorrowedName<'a>,
    age: i64,
    #[serde(borrow)]
    gender: Cow<'a, str>,
}

#[derive(Deserialize, Debug, PartialEq)]
struct BorrowedName<'a> {
    #[serde(borrow)]
    first: Cow<'a, str>,
    #[serde(borrow)]
    last: Cow<'a, str>,
    #[serde(borrow)]
    full: Cow<'a, str>,
}

// Mixed borrowed and owned fields
#[derive(Deserialize, Debug, PartialEq)]
struct BorrowedMixed<'a> {
    #[serde(borrow)]
    name: BorrowedNameMixed<'a>,
    age: i64,
    employment: BorrowedEmployment<'a>,
    #[serde(borrow)]
    gender: Cow<'a, str>,
}

#[derive(Deserialize, Debug, PartialEq)]
struct BorrowedNameMixed<'a> {
    #[serde(borrow)]
    first: Cow<'a, str>,
    #[serde(borrow)]
    last: Cow<'a, str>,
    // full name might be interpolated, so we use Cow
    #[serde(borrow)]
    full: Cow<'a, str>,
}

#[derive(Deserialize, Debug, PartialEq)]
struct BorrowedEmployment<'a> {
    employed: bool,
    #[serde(borrow)]
    name: Cow<'a, str>,
    #[serde(rename = "sinceYear")]
    since_year: i64,
}

#[test]
fn test_borrowed_basic() {
    let root_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let input = fs::read_to_string(format!("{root_dir}/assets/inputs/basic.corn")).unwrap();

    let config: BorrowedBasic = from_str(&input).unwrap();

    assert_eq!(config.foo, "bar");
}

#[test]
fn test_borrowed_string() {
    let root_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let input = fs::read_to_string(format!("{root_dir}/assets/inputs/string.corn")).unwrap();

    let config: BorrowedString = from_str(&input).unwrap();

    assert_eq!(config.foo, "bar");
    assert_eq!(config.bar, "\"\\\n\r\t");
    assert_eq!(config.baz, "a"); // Unicode escape \u{0061}
    assert_eq!(config.qux, "");
}

#[test]
fn test_borrowed_string_cow() {
    let root_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let input = fs::read_to_string(format!("{root_dir}/assets/inputs/string.corn")).unwrap();

    let config: BorrowedStringCow = from_str(&input).unwrap();

    assert_eq!(config.foo, "bar");
    assert_eq!(config.bar, "\"\\\n\r\t");
    assert_eq!(config.baz, "a");
    assert_eq!(config.qux, "");
}

#[test]
fn test_borrowed_nested() {
    let root_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let input = fs::read_to_string(format!("{root_dir}/assets/inputs/complex.corn")).unwrap();

    let config: BorrowedNested = from_str(&input).unwrap();

    assert_eq!(config.name.first, "John");
    assert_eq!(config.name.last, "Smith");
    assert_eq!(config.name.full, "John Smith");
    assert_eq!(config.age, 32);
    assert_eq!(config.gender, "M");
}

#[test]
fn test_borrowed_mixed() {
    let root_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let input = fs::read_to_string(format!("{root_dir}/assets/inputs/complex.corn")).unwrap();

    let config: BorrowedMixed = from_str(&input).unwrap();

    assert_eq!(config.name.first, "John");
    assert_eq!(config.name.last, "Smith");
    assert_eq!(config.name.full, "John Smith");
    assert_eq!(config.age, 32);
    assert!(config.employment.employed);
    assert_eq!(config.employment.name, "Postman");
    assert_eq!(config.employment.since_year, 2019);
    assert_eq!(config.gender, "M");
}

// Test that demonstrates zero-copy behavior with lifetime constraints
#[test]
fn test_borrowed_lifetime_constraint() {
    let root_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let input = fs::read_to_string(format!("{root_dir}/assets/inputs/basic.corn")).unwrap();

    let config: BorrowedBasic;

    {
        config = from_str(&input).unwrap();
        assert_eq!(config.foo, "bar");
    }

    // config can still be used here because input is still alive
    assert_eq!(config.foo, "bar");
}

// Test array with borrowed string elements
#[derive(Deserialize, Debug, PartialEq)]
struct BorrowedArray<'a> {
    #[serde(borrow)]
    foo: Vec<Cow<'a, str>>,
}

#[test]
fn test_borrowed_array() {
    let test_input = r#"{ foo = ["hello" "world" "test"] }"#;

    let config: BorrowedArray = from_str(test_input).unwrap();

    assert_eq!(config.foo.len(), 3);
    assert_eq!(config.foo[0], "hello");
    assert_eq!(config.foo[1], "world");
    assert_eq!(config.foo[2], "test");
}
