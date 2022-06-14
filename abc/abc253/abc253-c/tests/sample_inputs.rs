use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"8
1 3
1 2
3
1 2
1 7
3
2 2 3
3
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1\n5\n4\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"4
1 10000
1 1000
2 100 3
1 10
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "");
    assert!(output.stderr_str().is_empty());
}
