use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"5 3 5
1 3 4
3 3 1 1 2
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "2 4 5\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"2 2 2
1 2
1 2
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1 2\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"10 6 9
1 3 5 7 8 9
1 2 3 4 5 6 5 6 2
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "2 5 6 7 9 10\n");
    assert!(output.stderr_str().is_empty());
}

