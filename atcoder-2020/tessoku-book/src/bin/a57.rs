use proconio::{input, marker::Usize1};

// doubling

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [Usize1; n],
        xy: [(Usize1, usize); q],
    }

    let mut table = Vec::with_capacity(32);
    table.push(a.clone());
    for i in 0..31 {
        let mut v0 = Vec::with_capacity(n);
        for j in 0..n {
            v0.push(table[i][table[i][j]]);
        }
        table.push(v0);
    }

    for &(mut x, mut y) in &xy {
        let mut i = 0;
        while y > 0 {
            if y % 2 == 1 {
                x = table[i][x];
            }
            y /= 2;
            i += 1;
        }
        println!("{}", x + 1);
    }
}
