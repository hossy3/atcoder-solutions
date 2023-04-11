use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut a = vec![vec![false; n]; m];
    for i in 0..m {
        input! {
            x: [Usize1],
        }
        for j in x {
            a[i][j] = true;
        }
    }

    for i in 0..(n - 1) {
        for j in (i + 1)..n {
            let mut yes = false;
            for k in 0..m {
                if a[k][i] && a[k][j] {
                    yes = true;
                    break;
                }
            }
            if !yes {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
}
