use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }
    let mut x = vec![0; w];
    for i in 0..h {
        for j in 0..w {
            if c[i][j] == '#' {
                x[j] += 1;
            }
        }
    }

    for i in 0..(w - 1) {
        print!("{} ", x[i]);
    }
    println!("{}", x[w - 1]);
}
