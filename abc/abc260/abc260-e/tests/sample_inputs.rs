use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3 5
1 3
1 4
2 5
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "0 1 3 2 1\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"1 2
1 2
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "2 1\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"5 9
1 5
1 7
5 6
5 8
2 6
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "0 0 1 2 4 4 3 2 1\n");
    assert!(output.stderr_str().is_empty());
}

