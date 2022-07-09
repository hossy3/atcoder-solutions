use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"2 2 180
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "-2 -2\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"5 0 120
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "-2.49999999999999911182 4.33012701892219364908\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"0 0 11
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "0.00000000000000000000 0.00000000000000000000\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"15 5 360
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "15.00000000000000177636 4.99999999999999555911\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample5() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"-505 191 278
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "118.85878514480690171240 526.66743699786547949770\n");
    assert!(output.stderr_str().is_empty());
}
