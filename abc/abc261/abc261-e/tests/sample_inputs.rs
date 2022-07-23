use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3 10
3 3
2 5
1 12
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"9
15
12
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"9 12
1 1
2 2
3 3
1 4
2 5
3 6
1 7
2 8
3 9
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"0
2
1
0
5
3
3
11
2
"#);
    assert!(output.stderr_str().is_empty());
}
