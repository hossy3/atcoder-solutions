use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"6 1 0 2
80 60 80 60 70 70
40 20 50 90 90 80
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1\n4\n5\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"5 2 1 2
0 100 0 100 0
0 0 100 100 0
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1\n2\n3\n4\n5\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"15 4 3 2
30 65 20 95 100 45 70 85 20 35 95 50 40 15 85
0 25 45 35 65 70 80 90 40 55 20 20 45 75 100
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "2\n4\n5\n6\n7\n8\n11\n14\n15\n");
    assert!(output.stderr_str().is_empty());
}

