use std::collections::HashMap;

use proconio::input;

type Mint = ac_library::ModInt1000000007;

fn gcd(a: isize, b: isize) -> isize {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn main() {
    input! {
        n: usize,
    }

    let mut xy = vec![];
    for _ in 0..n {
        input! {
            k: usize,
            xy0: [(isize, isize); k],
        }
        xy.push(xy0);
    }

    // 凸図形同士のミンコフスキー和は再び凸図形となる なので、
    // 個々の要素が「点」「線」「凸図形」かを調べて、
    // 「1つ以上の点」「1つ以上の平行な線 + 0個以上の点」を全組み合わせから除外する

    let mut num_points = 0;
    let mut lines = HashMap::new();
    for xy in &xy {
        if xy.len() == 1 {
            num_points += 1;
            continue;
        }
        if (0..(xy.len() - 2)).any(|i| {
            let (x0, y0) = xy[i];
            let (x1, y1) = xy[i + 1];
            let (x2, y2) = xy[i + 2];
            let (dx0, dy0) = (x1 - x0, y1 - y0);
            let (dx1, dy1) = (x2 - x1, y2 - y1);
            dx0 * dy1 != dy0 * dx1 // 1組でも平行でなければ凸
        }) {
            continue;
        }

        let (x0, y0) = xy[0];
        let (x1, y1) = xy[1];
        let (dx, dy) = (x1 - x0, y1 - y0);
        let g = gcd(dx, dy);
        let g = if dx * g >= 0 { g } else { -g }; // dx / g を正に寄せる
        let (dx, dy) = (dx / g, dy / g);
        *lines.entry((dx, dy)).or_insert(0) += 1;
    }
    // eprintln!("{:?}", &lines);

    let mut result = Mint::new(2).pow(n as u64) - 1; // 全組み合わせ
    result -= Mint::new(2).pow(num_points as u64) - 1; // 1つ以上の点
    for (_, &num_lines) in &lines {
        // 1つ以上の平行な線 + 0個以上の点
        result -= (Mint::new(2).pow(num_lines as u64) - 1) * Mint::new(2).pow(num_points as u64);
    }

    println!("{result}");
}
