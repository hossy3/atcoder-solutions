use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"4
3 1 4 1
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "2\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"10
99999 99998 99997 99996 99995 99994 99993 99992 99991 99990
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "120\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"15
3 1 4 1 5 9 2 6 5 3 5 8 9 7 9
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "355\n");
    assert!(output.stderr_str().is_empty());
}

