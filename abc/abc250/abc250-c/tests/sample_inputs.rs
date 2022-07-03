use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"5 5
1
2
3
4
5
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1 2 3 5 4\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"7 7
7
7
7
7
7
7
7
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1 2 3 4 5 7 6\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"10 6
1
5
2
9
6
6
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1 2 3 4 5 7 6 8 10 9\n");
    assert!(output.stderr_str().is_empty());
}

