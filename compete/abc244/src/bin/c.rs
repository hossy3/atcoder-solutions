use std::{
    collections::HashSet,
    io::{stdin, stdout, BufReader, Write},
};

use proconio::{input, source::line::LineSource};

fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));

    input! {
        from &mut source,
        n: usize,
    }
    let mut set: HashSet<_> = (1..=(2 * n + 1)).collect();
    while let Some(&x) = set.iter().next() {
        println!("{}", x);
        stdout().flush().unwrap();
        set.remove(&x);

        input! {
            from &mut source,
            x: usize,
        }
        set.remove(&x);
    }
}
