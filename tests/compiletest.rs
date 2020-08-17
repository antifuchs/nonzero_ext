#[rustversion::any(nightly, beta)]
#[test]
fn compile_test_prerelease() {
    let t = trybuild::TestCases::new();
    t.pass("tests/run-pass/*.rs");
    t.compile_fail("tests/compile-fail_1.46/*.rs");
}

#[rustversion::any(since(1.46))]
#[test]
fn compile_test_since_1_46() {
    let t = trybuild::TestCases::new();
    t.pass("tests/run-pass/*.rs");
    t.compile_fail("tests/compile-fail_1.46/*.rs");
}

#[rustversion::before(1.46)] // nightly+beta has changed the format of the macro backtrace hint.
#[test]
fn compile_test_until_1_45() {
    let t = trybuild::TestCases::new();
    t.pass("tests/run-pass/*.rs");
    t.compile_fail("tests/compile-fail_1.45/*.rs");
}
