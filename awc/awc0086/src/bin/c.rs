use proconio::input;

fn main() {
    input! {
        _n: usize,
        m: usize,
        q: usize,
        mut b: [usize; m],
        lr: [(usize, usize); q],
    }

    b.sort_unstable(); // 並び替えても答えは変わらない
    let mut v = vec![false; m + 1]; // 座標圧縮した会議室の点灯具合

    for &(l, r) in &lr {
        let l0 = b.partition_point(|&x| x < l);
        let r0 = b.partition_point(|&x| x <= r);
        v[l0] = !v[l0];
        v[r0] = !v[r0];
        // eprintln!("{}, {}", l0, r0);
    }
    for i in 0..m {
        v[i + 1] = v[i] ^ v[i + 1];
    }
    let result = v.iter().filter(|&&b| b).count();
    println!("{result}");
}
