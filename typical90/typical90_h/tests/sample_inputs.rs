use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"10
attcordeer
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"4
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"41
btwogablwetwoiehocghiewobadegwhoihegnldir
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"2
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"140
aaaaaaaaaaaaaaaaaaaattttttttttttttttttttccccccccccccccccccccooooooooooooooooooooddddddddddddddddddddeeeeeeeeeeeeeeeeeeeerrrrrrrrrrrrrrrrrrrr
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"279999993
"#);
    assert!(output.stderr_str().is_empty());
}
