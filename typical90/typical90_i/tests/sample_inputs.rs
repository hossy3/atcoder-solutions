use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3
0 0
0 10
10 10
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"90
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"5
8 6
9 1
2 0
1 0
0 1
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"171.869897645844
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"10
0 0
1 7
2 6
2 8
3 5
5 5
6 7
7 1
7 9
8 8
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"180
"#);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"40
298750376 229032640
602876667 944779015
909539868 533609371
231368330 445484152
408704870 850216874
349286798 30417810
807260002 554049450
40706045 380488344
749325840 801881841
459457853 66691229
5235900 8100458
46697277 997429858
827651689 790051948
981897272 271364774
536232393 997361572
449659237 602191750
294800444 346669663
792837293 277667068
997282249 468293808
444906878 702693341
894286137 845317003
27053625 926547765
739689211 447395911
902031510 326127348
582956343 842918193
235655766 844300842
438389323 406413067
862896425 464876303
68833418 76340212
911399808 745744264
551223563 854507876
196296968 52144186
431165823 275217640
424495332 847375861
337078801 83054466
648322745 694789156
301518763 319851750
432518459 772897937
630628124 581390864
313132255 350770227
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), r#"179.9834340684232
"#);
    assert!(output.stderr_str().is_empty());
}
