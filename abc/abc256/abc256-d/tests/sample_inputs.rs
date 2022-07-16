use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3
10 20
20 30
40 50
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "10 30\n40 50\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3
10 40
30 60
20 50
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "10 60\n");
    assert!(output.stderr_str().is_empty());
}

