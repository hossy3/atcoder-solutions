use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [Usize1; n],
    }

    let mut v = vec![usize::MAX; n];
    let mut pos = 0; // 現在の位置
    let mut step = 0; // 現在の歩数
    v[pos] = step;
    while step < k {
        pos = a[pos];
        step += 1;
        if v[pos] == usize::MAX {
            v[pos] = step;
        } else {
            let l = step - v[pos]; // ループ長
            if (k - step) / l > 0 {
                step += (k - step) / l * l;
            }
        }
    }
    println!("{}", pos + 1);
}
