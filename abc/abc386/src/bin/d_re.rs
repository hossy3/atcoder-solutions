use proconio::{input, marker::Usize1};

fn f(x2yc: &[Vec<(usize, char)>]) -> bool {
    let n = x2yc.len();
    let mut y0 = n; // 白がはじまる場所
    for yc in x2yc {
        for &(y, c) in yc {
            match c {
                'B' => {
                    if y >= y0 {
                        return false;
                    }
                }
                'W' => {
                    y0 = y0.min(y);
                }
                _ => unreachable!(),
            }
        }
    }
    true
}

fn main() {
    input! {
        n: usize,
        m: usize,
        xyc: [(Usize1, Usize1, char); m],
    }

    let mut x2yc = vec![vec![]; n];
    let mut y2xc = vec![vec![]; n];
    for &(x, y, c) in &xyc {
        x2yc[x].push((y, c));
        y2xc[y].push((x, c));
    }
    for i in 0..n {
        x2yc[i].sort();
        y2xc[i].sort();
    }
    let yes = f(&x2yc) && f(&y2xc);
    println!("{}", if yes { "Yes" } else { "No" });
}
