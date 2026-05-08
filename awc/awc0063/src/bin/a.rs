use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        _w: usize,
        s: [Chars; h],
    }

    let mut v = vec![];
    for (i, s) in s.iter().enumerate() {
        for (j, &c) in s.iter().enumerate() {
            if c == 'T' {
                v.push((i + 1, j + 1));
            }
        }
    }

    println!("{}", v.len());
    for (i, j) in v {
        println!("{i} {j}");
    }
}
