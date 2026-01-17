use ac_library::FenwickTree;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut tree = FenwickTree::new(n * 2 + 3, 0usize);
    let mut offset = n + 1;
    let mut count = 0usize;
    for c in s {
        tree.add(offset, 1);
        match c {
            'A' => {
                offset -= 1;
            }
            'B' => {
                offset += 1;
            }
            _ => {} // noop
        }
        count += tree.sum((offset + 1)..);
    }
    println!("{count}");
}
