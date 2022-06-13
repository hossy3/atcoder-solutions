use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"6 5
2 3
3 4
3 5
5 6
2 6
7
1 1
2 2
2 0
2 3
4 1
6 0
4 3
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"1
20
2
20
7
6
20
"#);
    assert!(output.stderr_str().is_empty());
}
