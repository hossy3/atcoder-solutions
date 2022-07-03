use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3 3
abc
2 2
1 1
2 2
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "b\na\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"10 8
dsuccxulnl
2 4
2 7
1 2
2 7
1 1
1 2
1 3
2 5
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "c\nu\nc\nu\n");
    assert!(output.stderr_str().is_empty());
}
