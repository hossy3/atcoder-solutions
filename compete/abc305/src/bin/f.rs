use std::{
    collections::BTreeSet,
    io::{stdin, stdout, BufReader, Write},
};

use proconio::{input, marker::Usize1, source::line::LineSource};

// interactive

fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));

    input! {
        from &mut source,
        n: usize,
        _: usize,
    }

    let mut log = vec![0];
    let mut set = BTreeSet::new();
    set.insert(0);
    'outer: while !set.contains(&(n - 1)) {
        input! {
            from &mut source,
            k: usize,
            v: [Usize1; k],
        }
        for &i in &v {
            if !set.contains(&i) {
                set.insert(i);
                log.push(i);
                println!("{}", i + 1);
                stdout().flush().unwrap();
                continue 'outer;
            }
        }

        log.pop();
        let i = log[log.len() - 1];
        println!("{}", i + 1);
        stdout().flush().unwrap();
    }

    input! {
        from &mut source,
        _: String,
    }
}
