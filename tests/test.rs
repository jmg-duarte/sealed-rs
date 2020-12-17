#[test]
fn compile_test() {
    let t = trybuild::TestCases::new();
    t.pass("tests/compile/01.rs");
}