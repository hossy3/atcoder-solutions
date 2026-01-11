use proconio::input;

fn main() {
    input! {
        n: usize,
        whb: [(usize, isize, isize); n],
    }

    let m = whb.iter().map(|&(w, _, _)| w).sum::<usize>() / 2;
    let mut v = vec![-1; m + 1]; // 頭の重さに対する価値
    v[0] = 0;

    for &(w, h, b) in &whb {
        for i in (0..=m).rev() {
            if v[i] < 0 {
                continue;
            }
            let j = i + w;
            if j <= m {
                v[j] = v[j].max(v[i] + h);
            }
            v[i] += b;
        }
    }

    let result = *v.iter().max().unwrap();
    println!("{result}");
}
