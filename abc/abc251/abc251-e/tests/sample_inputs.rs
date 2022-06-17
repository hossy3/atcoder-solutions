use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"5
2 5 3 2 5
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "7\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"20
29 27 79 27 30 4 93 89 44 88 70 75 96 3 78 39 97 12 53 62
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "426\n");
    assert!(output.stderr_str().is_empty());
}
