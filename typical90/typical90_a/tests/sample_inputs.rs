use cli_test_dir::*;
const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"
        3 34
        1
        8 13 26
        "#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "13\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"
        7 45
        2
        7 11 16 20 28 34 38
        "#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "12\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"
        3 100
        1
        28 54 81
        "#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "46\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"
        3 100
        2
        28 54 81
        "#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "26\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample5() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"
        20 1000
        4
        51 69 102 127 233 295 350 388 417 466 469 523 553 587 720 739 801 855 926 954
        "#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "170\n");
    assert!(output.stderr_str().is_empty());
}
