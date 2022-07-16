use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3
1937458062
8124690357
2385760149
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "6\n");
    assert!(output.stderr_str().is_empty());
}

// #[test]
// fn sample2() {
//     let testdir = TestDir::new(BIN, "");
//     let output = testdir
//         .cmd()
//         .output_with_stdin(r#"5
// 0123456789
// 0123456789
// 0123456789
// 0123456789
// 0123456789
// "#)
//         .tee_output()
//         .expect_success();
//     assert_eq!(output.stdout_str(), "40\n");
//     assert!(output.stderr_str().is_empty());
// }
