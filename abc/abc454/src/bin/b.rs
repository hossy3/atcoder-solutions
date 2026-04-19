use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        f: [Usize1; n],
    }

    let mut yes = true; // 全部異なる服？
    let mut v = vec![false; m];
    for &f in &f {
        if v[f] {
            yes = false;
        } else {
            v[f] = true;
        }
    }
    println!("{}", if yes { "Yes" } else { "No" });

    let yes = v.iter().all(|&x| x); // M 種類の服をすべてきている？
    println!("{}", if yes { "Yes" } else { "No" });
}
