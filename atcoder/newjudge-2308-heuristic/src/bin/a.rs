use std::io::{stdin, stdout, BufReader, Write};

use itertools::Itertools;
use proconio::{input, source::line::LineSource};

// interactive

fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));

    input! {
        from &mut source,
        d: usize,
        c: [i64; 26],
    }

    let mut c0 = vec![0i64; 26];
    for _ in 0..d {
        input! {
            from &mut source,
            s: [i64; 26],
        }
        for j in 0..26 {
            c0[j] += c[j];
        }
        let j = (0..26)
            .position_max_by_key(|&j| c0[j] * 11 - c[j] + s[j])
            .unwrap();
        c0[j] = 0;
        println!("{}", j + 1);
        stdout().flush().unwrap();
    }
}
