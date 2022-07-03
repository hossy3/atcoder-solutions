use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"5
1 2 3 4 5
1 2 2 4 3
7
1 1
2 2
2 3
3 3
4 4
4 5
5 5
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"Yes
Yes
Yes
No
No
Yes
No
"#);
    assert!(output.stderr_str().is_empty());
}
