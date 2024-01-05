use std::io::{stdin, stdout, BufReader, Write};

use proconio::{input, source::line::LineSource};

fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));

    input! {
        from &mut source,
        n: usize,
        _: usize,
    }

    let mut log = vec![1];
    let mut visited = vec![false; n + 1];
    visited[1] = true;

    loop {
        input! {
            from &mut source,
            k: usize,
            v: [usize; k],
        }
        if v.contains(&n) {
            break;
        }
        if let Some(&i) = v.iter().find(|&&i| !visited[i]) {
            visited[i] = true;
            log.push(i);
            println!("{}", i);
        } else {
            log.pop();
            let i = *log.last().unwrap();
            println!("{}", i);
        }
    }

    println!("{}", n);
    stdout().flush().unwrap();

    input! {
        from &mut source,
        s: String,
    }
    assert_eq!(s, "OK");
}
