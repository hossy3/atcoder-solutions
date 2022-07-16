use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"7 3
atcoder
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "acd\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"14 5
kittyonyourlap
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "inlap\n");
    assert!(output.stderr_str().is_empty());
}
