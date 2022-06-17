use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"6"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "3\n1 2 3\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"12"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "5\n1 2 3 5 8\n");
    assert!(output.stderr_str().is_empty());
}
