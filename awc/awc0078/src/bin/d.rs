use ac_library::Dsu;
use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        ab: [(usize, usize); n],
        k: [usize; q],
    }

    // イベントソート
    let mut events = Vec::new();
    for (i, &k) in k.iter().enumerate() {
        events.push((k, 0, i, 0));
    }
    for (i, &(a, b)) in ab.iter().enumerate() {
        events.push((a, 1, i, b));
    }
    events.sort();

    let mut dsu = Dsu::new(n);
    let mut v = vec![0usize; n]; // 重さの最大値
    let mut results = vec![0usize; q]; // 問い合わせに対するスコア
    let mut cur = 0; // 現在のスコア
    for (_, t, i, b) in events {
        match t {
            0 => {
                results[i] = cur;
            }
            1 => {
                if i > 0 && v[dsu.leader(i - 1)] > 0 {
                    if i < n - 1 && v[dsu.leader(i + 1)] > 0 {
                        let i0 = dsu.leader(i - 1);
                        let i1 = dsu.leader(i + 1);
                        let b = b.max(v[i0]).max(v[i1]);
                        cur += b;
                        cur -= v[i0] + v[i1];
                        dsu.merge(i0, i);
                        dsu.merge(i, i1);
                        let i = dsu.leader(i);
                        v[i] = b;
                    } else {
                        let i0 = dsu.leader(i - 1);
                        let b = b.max(v[i0]);
                        cur += b;
                        cur -= v[i0];
                        dsu.merge(i0, i);
                        let i = dsu.leader(i);
                        v[i] = b;
                    }
                } else {
                    if i < n - 1 && v[dsu.leader(i + 1)] > 0 {
                        let i1 = dsu.leader(i + 1);
                        let b = b.max(v[i1]);
                        cur += b;
                        cur -= v[i1];
                        dsu.merge(i, i1);
                        let i = dsu.leader(i);
                        v[i] = b;
                    } else {
                        cur += b;
                        v[i] = b;
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    for result in results {
        println!("{result}");
    }
}
