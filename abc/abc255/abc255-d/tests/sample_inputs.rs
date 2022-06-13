use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"5 3
6 11 2 5 5
5
20
0
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "10\n71\n29\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"10 5
1000000000 314159265 271828182 141421356 161803398 0 777777777 255255255 536870912 998244353
555555555
321654987
1000000000
789456123
0
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "3316905982\n2811735560\n5542639502\n4275864946\n4457360498\n");
    assert!(output.stderr_str().is_empty());
}
