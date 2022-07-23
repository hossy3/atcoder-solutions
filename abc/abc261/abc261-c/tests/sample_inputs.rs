use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"5
newfile
newfile
newfolder
newfile
newfolder
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"newfile
newfile(1)
newfolder
newfile(2)
newfolder(1)
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"11
a
a
a
a
a
a
a
a
a
a
a
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"a
a(1)
a(2)
a(3)
a(4)
a(5)
a(6)
a(7)
a(8)
a(9)
a(10)
"#);
    assert!(output.stderr_str().is_empty());
}
