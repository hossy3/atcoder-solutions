use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        fs: [(Usize1, usize); n],
    }

    let mut v = vec![(0, 0); n]; // top 2
    for &(f, s) in &fs {
        if v[f].0 < s {
            v[f].1 = v[f].0;
            v[f].0 = s;
        } else if v[f].1 < s {
            v[f].1 = s;
        }
    }
    
    v.sort();
    let mut result = v[n - 1].0 + v[n - 2].0;
    for &(x0, x1) in &v {
        result = result.max(x0 + x1 / 2);
    }

    println!("{result}");
}
