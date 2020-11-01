// TODO: parallel running of trybuild tests doesn't work.
// Unfortunately, we can't have a single test for successful
// compilation of various macro invocations. This is prevented by
// https://github.com/dtolnay/trybuild/issues/58 - trybuild confuses
// about which test case should be successful or not. Instead, just
// run the t.pass and t.compile_fail calls in the same test case - we
// lose parallelism but at least it works.

#[rustversion::any(nightly)]
#[test]
fn compile_test_prerelease() {
    let t = trybuild::TestCases::new();
    t.pass("tests/run-pass/*.rs");
    t.compile_fail("tests/compile-fail_nightly/*.rs");
}

#[rustversion::before(1.46)] // nightly+beta has changed the format of the macro backtrace hint.
#[test]
fn compile_test_until_1_45() {
    let t = trybuild::TestCases::new();
    t.pass("tests/run-pass/*.rs");
    t.compile_fail("tests/compile-fail_1.45/*.rs");
}

#[rustversion::all(since(1.46), before(1.48))]
#[test]
fn compile_test_since_1_46() {
    let t = trybuild::TestCases::new();
    t.pass("tests/run-pass/*.rs");
    t.compile_fail("tests/compile-fail_1.46/*.rs");
}

#[rustversion::all(since(1.48), not(nightly))]
#[test]
fn compile_test_stable_since_1_48() {
    let t = trybuild::TestCases::new();
    t.pass("tests/run-pass/*.rs");
    t.compile_fail("tests/compile-fail_1.48/*.rs");
}
