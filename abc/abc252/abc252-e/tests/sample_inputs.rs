use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3 3
1 2 1
2 3 2
1 3 10
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1 2\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"4 6
1 2 1
1 3 1
1 4 1
2 3 1
2 4 1
3 4 1
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "3 1 2\n");
    assert!(output.stderr_str().is_empty());
}
