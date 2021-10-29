#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.pass("tests/cases/option_enum.rs");
}
