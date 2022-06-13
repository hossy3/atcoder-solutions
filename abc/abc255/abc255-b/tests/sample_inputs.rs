use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"4 2
2 3
0 0
0 1
1 2
2 0
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "2.23606797749978969\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"2 1
2
-100000 -100000
100000 100000
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "282842.712474619009\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"8 3
2 6 8
-17683 17993
93038 47074
58079 -57520
-41515 -89802
-72739 68805
24324 -73073
71049 72103
47863 19268
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "130379.280458974768\n");
    assert!(output.stderr_str().is_empty());
}

