#[test]
fn compile_test() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile/01.rs");
}
