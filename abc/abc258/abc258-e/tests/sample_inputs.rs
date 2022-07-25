use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3 2 5
3 4 1
1
2
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "2\n3\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"10 5 20
5 8 5 9 8 7 4 4 8 2
1
1000
1000000
1000000000
1000000000000
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "4\n5\n5\n5\n5\n");
    assert!(output.stderr_str().is_empty());
}
