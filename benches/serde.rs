use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use serde::Deserialize;
use std::hint::black_box;

#[derive(Deserialize, Debug, PartialEq)]
struct Empty {}

#[derive(Deserialize, Debug, PartialEq)]
struct Array {
    foo: Vec<i64>,
}

#[derive(Deserialize, Debug, PartialEq)]
struct Basic {
    foo: String,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
enum BasicNewTypeEnum {
    Foo(String),
}

#[derive(Deserialize, Debug, PartialEq)]
struct BasicUnitEnum {
    foo: BasicUnitEnumInner,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
enum BasicUnitEnumInner {
    Bar,
}

#[derive(Deserialize, Debug, PartialEq)]
struct BasicNewType {
    foo: BasicNewTypeInner,
}

#[derive(Deserialize, Debug, PartialEq)]
struct BasicNewTypeInner(String);

#[derive(Deserialize, Debug, PartialEq)]
struct Boolean {
    foo: bool,
    bar: bool,
}

#[derive(Deserialize, Debug, PartialEq)]
struct Bytes<'a> {
    foo: &'a [u8],
}

#[derive(Deserialize, Debug, PartialEq)]
struct Chained {
    foo: ChainedInner,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ChainedInner {
    bar: String,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
enum ChainedEnum {
    Foo { bar: String },
}

#[derive(Deserialize, Debug, PartialEq)]
struct ChainedComplex {
    foo: ChainedComplexInner,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ChainedComplexInner {
    bar: ChainedComplexInnerInner,
    qux: bool,
    quux: Vec<String>,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ChainedComplexInnerInner {
    baz: i64,
}

#[derive(Deserialize, Debug, PartialEq)]
struct Char {
    foo: char,
}

#[derive(Deserialize, Debug, PartialEq)]
struct Compact {
    one: CompactOne,
    two: CompactTwo,
    three: CompactThree,
    four: CompactFour,
    five: CompactFive,
    six: CompactSix,
    seven: CompactSeven,
    eight: Vec<String>,
    nine: Vec<bool>,
    ten: Vec<()>,
    eleven: Vec<Vec<u8>>,
    twelve: Vec<Empty>,
}

#[derive(Deserialize, Debug, PartialEq)]
struct CompactOne {
    foo: String,
    bar: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct CompactTwo {
    foo: i64,
    bar: i64,
}

#[derive(Deserialize, Debug, PartialEq)]
struct CompactThree {
    foo: f64,
    bar: f64,
}

#[derive(Deserialize, Debug, PartialEq)]
struct CompactFour {
    foo: bool,
    bar: bool,
}

#[derive(Deserialize, Debug, PartialEq)]
struct CompactFive {
    foo: (),
    bar: (),
}

#[derive(Deserialize, Debug, PartialEq)]
struct CompactSix {
    foo: Empty,
    bar: Empty,
}

#[derive(Deserialize, Debug, PartialEq)]
struct CompactSeven {
    foo: Vec<u8>,
    bar: Vec<u8>,
}

#[derive(Deserialize, Debug, PartialEq)]
struct Complex {
    age: i64,
    employment: ComplexEmployment,
    empty1: Empty,
    empty2: Vec<u8>,
    favourites: (String, String, String, String, f64, bool, Favourites),
    gender: String,
    name: ComplexName,
    negative: ComplexNegative,
    parents: ComplexParents,
    placeholder: (),
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
struct ComplexEmployment {
    employed: bool,
    name: String,
    since_year: i64,
}

#[derive(Deserialize, Debug, PartialEq)]
struct Favourites {
    food: ComplexFavouritesFood,
    hello: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ComplexFavouritesFood {
    favourite: String,
    hated: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ComplexName {
    first: String,
    full: String,
    last: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ComplexNegative {
    float: f64,
    int: i64,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ComplexParents {
    father: ComplexParentsFather,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ComplexParentsFather {
    birthday: ComplexParentsFatherBirthday,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ComplexParentsFatherBirthday {
    day: i64,
    month: i64,
    year: i64,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ComplexKeys {
    #[serde(rename = "!\"£$%^&*()_")]
    symbols: i64,
    #[serde(rename = "apple-pie")]
    apple_pie: ComplexKeysApplePie,
    foo: ComplexKeysFoo,
    j12345: i64,
    #[serde(rename = "with-dash")]
    with_dash: i64,
    #[serde(rename = "with_underscore")]
    with_underscore: i64,
    #[serde(rename = "with_🌽")]
    with_corn_emoji: i64,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ComplexKeysApplePie {
    crust: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ComplexKeysFoo {
    #[serde(rename = "bar-baz")]
    bar_baz: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct Float {
    foo: f64,
    bar: f64,
}

#[derive(Deserialize, Debug, PartialEq)]
struct Input {
    name: InputName,
    dob: InputDob,
}

#[derive(Deserialize, Debug, PartialEq)]
struct InputName {
    first: String,
    last: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct InputDob {
    day: u8,
    month: u8,
    year: u16,
}

#[derive(Deserialize, Debug, PartialEq)]
struct Integer {
    foo: i64,
}

#[derive(Deserialize, Debug, PartialEq)]
struct MixedArray {
    foo: (u8, String, bool),
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
enum MixedArrayEnum {
    Foo(u8, String, bool),
}

#[derive(Deserialize, Debug, PartialEq)]
struct Null {
    foo: (),
}

#[derive(Deserialize, Debug, PartialEq)]
struct NullOption {
    foo: Option<u8>,
}

#[derive(Deserialize, Debug, PartialEq)]
struct NullUnit {
    foo: NullUnitInner,
}

#[derive(Deserialize, Debug, PartialEq)]
struct NullUnitInner;

#[derive(Deserialize, Debug, PartialEq)]
struct Object {
    foo: SubObject,
}

#[derive(Deserialize, Debug, PartialEq)]
struct SubObject {
    bar: i64,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ObjectInArray {
    foo: Vec<ObjectInArrayFoo>,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ObjectInArrayFoo {
    bar: i64,
    foo: i64,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
struct ReadmeExample {
    author: ReadmeExampleAuthor,
    bin: ReadmeExampleBin,
    config: ReadmeExampleConfig,
    contributors: Vec<ReadmeExampleContributor>,
    dependencies: ReadmeExampleDependencies,
    dev_dependencies: ReadmeExampleDevDependencies,
    main: String,
    name: String,
    private: bool,
    scripts: ReadmeExampleScripts,
    version: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct String_ {
    foo: String,
    bar: String,
    baz: String,
    qux: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ReadmeExampleAuthor {
    email: String,
    name: String,
    url: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ReadmeExampleBin {
    filebrowser: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ReadmeExampleConfig {
    hostname: Option<String>,
    port: i64,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ReadmeExampleContributor {
    email: String,
    name: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ReadmeExampleDependencies {
    dotenv: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ReadmeExampleDevDependencies {
    typescript: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ReadmeExampleScripts {
    build: String,
    run: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct Str<'a> {
    foo: &'a str,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ValueAfterTable {
    foo: Empty,
    qux: bool,
}

macro_rules! bench_case {
    ($group:expr, $name:literal, $input_file:literal, $output_type:ty) => {
        let input = include_str!(concat!("../assets/inputs/", $input_file, ".corn"));
        $group.throughput(Throughput::Bytes(input.len() as u64));
        $group.bench_function($name, |b| {
            b.iter(|| {
                let result: Result<$output_type, _> = corn::from_str(black_box(input));
                black_box(result.unwrap())
            })
        });
    };
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("corn_parsing");
    group.sample_size(500);

    bench_case!(group, "array", "array", Array);
    bench_case!(group, "basic", "basic", Basic);
    bench_case!(group, "basic_empty_let", "basic_empty_let", Basic);
    bench_case!(group, "boolean", "boolean", Boolean);
    bench_case!(group, "chained", "chained", Chained);
    bench_case!(group, "chained_complex", "chained_complex", ChainedComplex);
    bench_case!(group, "char", "char", Char);
    bench_case!(group, "comment", "comment", Basic);
    bench_case!(group, "compact", "compact", Compact);
    bench_case!(group, "complex", "complex", Complex);
    bench_case!(group, "complex_keys", "complex_keys", ComplexKeys);
    bench_case!(group, "environment_variable", "environment_variable", Basic);
    bench_case!(group, "float", "float", Float);
    bench_case!(group, "input", "input", Input);
    bench_case!(
        group,
        "input_references_input",
        "input_references_input",
        Basic
    );
    bench_case!(group, "integer", "integer", Integer);
    bench_case!(group, "mixed_array", "mixed_array", MixedArray);
    bench_case!(group, "null", "null", Null);
    bench_case!(group, "object", "object", Object);
    bench_case!(group, "object_in_array", "object_in_array", ObjectInArray);
    bench_case!(group, "readme_example", "readme_example", ReadmeExample);
    bench_case!(group, "string", "string", String_);
    bench_case!(group, "string_interpolation", "string_interpolation", Basic);
    bench_case!(
        group,
        "value_after_table",
        "value_after_table",
        ValueAfterTable
    );
    bench_case!(group, "very_compact", "very_compact", Compact);

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
