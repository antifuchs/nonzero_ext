#[rustversion::any(stable, beta)] // nightly has changed the format of the macro backtrace hint.
#[test]
fn compile_test() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile-fail/*.rs");
    t.pass("tests/run-pass/*.rs");
}
