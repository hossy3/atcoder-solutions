use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"6 3
2 7 1 8 2 8
2 10
3 1
5 5
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "48\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3 2
1000000000 1000000000 1000000000
1 1000000000
3 1000000000
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "5000000000\n");
    assert!(output.stderr_str().is_empty());
}
