use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"5 2
3 5 2 1 4
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "4\n3\n3\n-1\n4\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"5 1
1 2 3 4 5
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1\n2\n3\n4\n5\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"15 3
3 14 15 9 2 6 5 13 1 7 10 11 8 12 4
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "9\n9\n9\n15\n15\n6\n-1\n-1\n6\n-1\n-1\n-1\n-1\n6\n15\n");
    assert!(output.stderr_str().is_empty());
}
