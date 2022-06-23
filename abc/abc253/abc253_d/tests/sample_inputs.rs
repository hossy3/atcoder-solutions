use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"10 3 5"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "22\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"1000000000 314 159"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "495273003954006262\n");
    assert!(output.stderr_str().is_empty());
}
