use std::io::{stdin, stdout, BufReader, Write};

use proconio::{input, source::line::LineSource};

fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));

    input! {
        from &mut source,
        n: usize,
    }

    let mut i = 0;
    let mut k = ((n - 1) as f64).log2().ceil() as usize;
    while k > 0 {
        let j = i + (1 << (k - 1));
        if j < n {
            println!("? {}", j + 1);
            stdout().flush().unwrap();

            input! {
                from &mut source,
                x: usize,
            }
            if x == 0 {
                i = j;
            }
        }
        k -= 1;
    }

    println!("! {}", i + 1);
    stdout().flush().unwrap();
}
