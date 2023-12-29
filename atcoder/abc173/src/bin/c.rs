use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        c: [Chars; h],
    }

    let mut result = 0;
    for h0 in 0..(1 << h) {
        for w0 in 0..(1 << w) {
            let mut count = 0;
            for i in 0..h {
                for j in 0..w {
                    if c[i][j] == '#' && ((1 << i) & h0 == 0) && ((1 << j) & w0 == 0) {
                        count += 1;
                    }
                }
            }
            if count == k {
                result += 1;
            }
        }
    }
    println!("{result}");
}
