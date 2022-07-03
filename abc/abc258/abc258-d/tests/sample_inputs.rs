use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3 4
3 4
2 3
4 2
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "18\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"10 1000000000
3 3
1 6
4 7
1 8
5 7
9 9
2 4
6 4
5 1
3 1
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1000000076\n");
    assert!(output.stderr_str().is_empty());
}
