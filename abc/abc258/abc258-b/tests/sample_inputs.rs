use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"4
1161
1119
7111
1811
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "9786\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"10
1111111111
1111111111
1111111111
1111111111
1111111111
1111111111
1111111111
1111111111
1111111111
1111111111
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1111111111\n");
    assert!(output.stderr_str().is_empty());
}
