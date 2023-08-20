use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h],
        b: [Chars; h],
    }
    let f = || {
        for s in 0..h {
            for t in 0..w {
                let result = (0..h).all(|i| {
                    (0..w).all(|j| {
                        let i0 = (i + s) % h;
                        let j0 = (j + t) % w;
                        a[i0][j0] == b[i][j]
                    })
                });
                if result {
                    return true;
                }
            }
        }
        false
    };
    let yes = f();
    println!("{}", if yes { "Yes" } else { "No" });
}
