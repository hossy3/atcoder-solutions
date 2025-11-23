use std::time::Instant;

use proconio::input;

const LIMIT_TIME: f64 = 1.7;

fn f(h: &[usize], c: &[usize], a: &[Vec<usize>], threshold: usize) -> Vec<(isize, usize)> {
    let n = h.len();

    let mut h: Vec<usize> = h.iter().copied().collect(); // 宝箱の耐久値
    let mut c: Vec<usize> = c.iter().copied().collect(); // 武器の耐久値
    let mut wb = vec![];

    let mut rest_h: usize = h.iter().sum();
    while rest_h > 0 {
        if (0..n).all(|i| h[i] > 0 || c[i] == 0) || (0..n).any(|i| 0 < h[i] && h[i] <= threshold) {
            // 武器が使えない または 使う必要がない場合、一番耐久値の低い宝箱を手で壊す
            let mut b = usize::MAX;
            for (b0, &h0) in h.iter().enumerate() {
                if h0 > 0 && (b == usize::MAX || h0 < h[b]) {
                    b = b0;
                }
            }
            for _ in 0..h[b] {
                wb.push((-1, b));
            }
            // eprintln!("{} {}", rest_h, h[b]);
            rest_h -= h[b];
            h[b] = 0;
            continue;
        }

        // 使える武器の中で一番効果的なものを採用する
        let mut cand = (isize::MAX, usize::MAX, usize::MAX, 0, 0); // (消した量, rest, power, w, b)
        for w in 0..n {
            if h[w] > 0 || c[w] == 0 {
                continue; // 武器を使えない
            }
            for (b, &h0) in h.iter().enumerate() {
                if h0 == 0 {
                    continue;
                }

                let rest = a[w][b].saturating_sub(h0);
                let mut power = 0; // ほかの宝箱を壊す力の最大値。小さいほど優先したい
                for (b0, &a0) in a[w].iter().enumerate() {
                    if b0 == b {
                        continue;
                    }
                    power = power.max(h[b0].min(a0));
                }
                let cand0 = (rest as isize - a[w][b] as isize, rest, power, w, b);
                if cand > cand0 {
                    cand = cand0;
                }
            }
        }

        let (_, _, _, w, b) = cand;
        wb.push((w as isize, b));
        let h0 = h[b].saturating_sub(a[w][b]);
        rest_h -= h[b] - h0;
        h[b] = h0;
        c[w] -= 1;
    }

    wb
}

fn main() {
    input! {
        n: usize,
        h: [usize; n],
        c: [usize; n],
        a: [[usize; n]; n],
    }

    let timer = Instant::now();

    let mut wb = vec![];
    for i in 1..25 {
        let wb0 = f(&h, &c, &a, i);
        if wb.len() == 0 || wb.len() > wb0.len() {
            wb = wb0;
        }
        if timer.elapsed().as_secs_f64() >= LIMIT_TIME {
            break;
        }
    }
    for (w, b) in wb {
        println!("{w} {b}");
    }
}
