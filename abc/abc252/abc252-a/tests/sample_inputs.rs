use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin("97")
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "a\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin("122")
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "z\n");
    assert!(output.stderr_str().is_empty());
}
