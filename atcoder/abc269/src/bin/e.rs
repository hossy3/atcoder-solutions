use std::io::{stdin, stdout, BufReader, Write};

use proconio::{input, source::line::LineSource};

// interactive

fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));

    input! {
        from &mut source,
        n: usize,
    }

    let mut i0 = 1;
    let mut i1 = n;
    while i0 < i1 {
        let i = (i1 + i0) / 2;
        println!("? {} {} {} {}", i0, i, 1, n);
        stdout().flush().unwrap();
        input! {
            from &mut source,
            t: usize,
        }
        if t == i - i0 {
            i1 = i;
        } else {
            i0 = i + 1;
        }
    }

    let mut j0 = 1;
    let mut j1 = n;
    while j0 < j1 {
        let j = (j1 + j0) / 2;
        println!("? {} {} {} {}", 1, n, j0, j);
        stdout().flush().unwrap();
        input! {
            from &mut source,
            t: usize,
        }
        if t == j - j0 {
            j1 = j;
        } else {
            j0 = j + 1;
        }
    }

    println!("! {} {}", i0, j0);
    stdout().flush().unwrap();
}
