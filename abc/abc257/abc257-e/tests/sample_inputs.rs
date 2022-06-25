use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"5
5 4 3 3 2 5 3 5 3
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "95\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"20
1 1 1 1 1 1 1 1 1
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "99999999999999999999\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"5
2 3 4 5 6 7 8 9 10
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "21\n");
    assert!(output.stderr_str().is_empty());
}
