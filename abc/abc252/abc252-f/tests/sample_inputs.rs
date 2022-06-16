use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"5 7
1 2 1 2 1
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "16\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3 1000000000000000
1000000000 1000000000 1000000000
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1000005000000000\n");
    assert!(output.stderr_str().is_empty());
}
