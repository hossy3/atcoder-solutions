use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"38 20 17 168 3
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "168\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"1 0 1 3 2
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"100 10 100 180 1
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "90\n");
    assert!(output.stderr_str().is_empty());
}

