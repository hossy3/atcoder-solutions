use ac_library::{Min, Segtree};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        abc: [(isize, isize, isize); n],
        ls: [(Usize1, isize); q],
    }

    let mut sum = vec![0; n + 1]; // 0日目からいくら資金が変わるか
    for (i, &(a, b, c)) in abc.iter().enumerate() {
        let x = a - b - c;
        sum[i + 1] = sum[i] + x;
    }
    let segtree: Segtree<Min<_>> = sum.clone().into();
    let min = segtree.all_prod();

    let mut results = vec![];
    for &(l, mut s) in &ls {
        let l = l % n; // ループの何日目か

        // ループの終わりまでに資金が枯渇するか
        let r = segtree.max_right(l, |&x| (s - sum[l]).saturating_add(x) >= 0);
        if r < n {
            results.push(r - l);
            continue;
        }
        s = s + sum[n] - sum[l];

        // ループを繰り返して資金が増える場合は1周だけ確認する
        if sum[n] >= 0 {
            let r = segtree.max_right(0, |&x| s.saturating_add(x) >= 0);
            if r < n {
                results.push((n - l) + r);
            } else {
                results.push(0);
            }
            continue;
        }

        // s + loops * sum[n] + min < 0 となる最初の loop: loops > (s + min) / -sum[n]
        let loops = ((s + min - sum[n]) / -sum[n]).max(0);
        s += loops * sum[n]; // この間は資金が枯渇しないはず
        let r = segtree.max_right(0, |&x| s.saturating_add(x) >= 0);
        results.push((n - l) + n * loops as usize + r);
    }

    for result in results {
        println!("{result}");
    }
}
