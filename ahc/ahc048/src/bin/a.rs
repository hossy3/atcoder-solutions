use std::collections::BinaryHeap;

use itertools::Itertools;
use proconio::input;

// すべて仕切りなし
fn build_pallet_v(n: usize) -> Vec<Vec<usize>> {
    vec![vec![0; n - 1]; n]
}

// ジグザグに仕切りを付ける
fn build_pallet_h(n: usize) -> Vec<Vec<usize>> {
    let mut v = vec![vec![1; n]; n - 1];
    for i in 0..(n - 1) {
        if i % 2 == 0 {
            v[i][n - 1] = 0;
        } else {
            v[i][0] = 0;
        }
    }
    v
}

const K: usize = 10000;
const TOL_ZERO: f64 = 1.0e-8;

fn build_ops_123(v: &[usize]) -> Vec<Vec<usize>> {
    let n = v.len();
    let mut ops = Vec::with_capacity(n * 2);
    for &i in v {
        ops.push(vec![1, 0, 0, i]); // 絵の具を1グラム出す
    }
    ops.push(vec![2, 0, 0]); // 画伯に渡す
    for _ in 1..n {
        ops.push(vec![3, 0, 0]); // 廃棄する
    }
    ops
}

// 仕切り移動なし - 絵具1回
fn update_candidates_123_1(
    d: usize,
    cmy_own: &[(f64, f64, f64)],
    (c_t, m_t, y_t): (f64, f64, f64),
    v: &mut Vec<(f64, Vec<Vec<usize>>)>,
) {
    let k = 1.0;
    let n = cmy_own.len();
    for i0 in 0..n {
        let (c, m, y) = cmy_own[i0];
        let dist2 = (c - c_t).powi(2) + (m - m_t).powi(2) + (y - y_t).powi(2);
        let score = dist2.sqrt() * K as f64 + (k - 1.0) * d as f64;
        let ops = build_ops_123(&[i0]);
        let j = ops.len();
        if v[j].0 > score {
            v[j] = (score, ops);
        }
    }
}

// 仕切り移動なし - 絵具2回
fn update_candidates_123_2(
    d: usize,
    cmy_own: &[(f64, f64, f64)],
    (c_t, m_t, y_t): (f64, f64, f64),
    v: &mut Vec<(f64, Vec<Vec<usize>>)>,
) {
    let k = 2.0;
    let n = cmy_own.len();
    for i0 in 0..n {
        let (c0, m0, y0) = cmy_own[i0];
        for i1 in i0..n {
            let (c1, m1, y1) = cmy_own[i1];
            let (c, m, y) = ((c0 + c1) / k, (m0 + m1) / k, (y0 + y1) / k);
            let dist2 = (c - c_t).powi(2) + (m - m_t).powi(2) + (y - y_t).powi(2);
            let score = dist2.sqrt() * K as f64 + (k - 1.0) * d as f64;
            let ops = build_ops_123(&[i0, i1]);
            let j = ops.len();
            if v[j].0 > score {
                v[j] = (score, ops);
            }
        }
    }
}

// 仕切り移動なし - 絵具3回
fn update_candidates_123_3(
    d: usize,
    cmy_own: &[(f64, f64, f64)],
    (c_t, m_t, y_t): (f64, f64, f64),
    v: &mut Vec<(f64, Vec<Vec<usize>>)>,
) {
    let k = 3.0;
    let n = cmy_own.len();
    for i0 in 0..n {
        let (c0, m0, y0) = cmy_own[i0];
        for i1 in i0..n {
            let (c1, m1, y1) = cmy_own[i1];
            for i2 in i1..n {
                let (c2, m2, y2) = cmy_own[i2];
                let (c, m, y) = ((c0 + c1 + c2) / k, (m0 + m1 + m2) / k, (y0 + y1 + y2) / k);
                let dist2 = (c - c_t).powi(2) + (m - m_t).powi(2) + (y - y_t).powi(2);
                let score = dist2.sqrt() * K as f64 + (k - 1.0) * d as f64;
                let ops = build_ops_123(&[i0, i1, i2]);
                let j = ops.len();
                if v[j].0 > score {
                    v[j] = (score, ops);
                }
            }
        }
    }
}

// 仕切り移動なし - 絵具4回
fn update_candidates_123_4(
    d: usize,
    cmy_own: &[(f64, f64, f64)],
    (c_t, m_t, y_t): (f64, f64, f64),
    v: &mut Vec<(f64, Vec<Vec<usize>>)>,
) {
    let k = 4.0;
    let n = cmy_own.len();
    for i0 in 0..n {
        let (c0, m0, y0) = cmy_own[i0];
        for i1 in i0..n {
            let (c1, m1, y1) = cmy_own[i1];
            for i2 in i1..n {
                let (c2, m2, y2) = cmy_own[i2];
                for i3 in i2..n {
                    let (c3, m3, y3) = cmy_own[i3];
                    let (c, m, y) = (
                        (c0 + c1 + c2 + c3) / k,
                        (m0 + m1 + m2 + m3) / k,
                        (y0 + y1 + y2 + y3) / k,
                    );
                    let dist2 = (c - c_t).powi(2) + (m - m_t).powi(2) + (y - y_t).powi(2);
                    let score = dist2.sqrt() * K as f64 + (k - 1.0) * d as f64;
                    let ops = build_ops_123(&[i0, i1, i2, i3]);
                    let j = ops.len();
                    if v[j].0 > score {
                        v[j] = (score, ops);
                    }
                }
            }
        }
    }
}

fn get_pos(n: usize, i: usize) -> (usize, usize) {
    let j = i / n;
    if j % 2 == 0 {
        (j, i % n)
    } else {
        (j, n - (i % n) - 1)
    }
}

// p: 0.0 ~ 1.0
fn get_divider_pos(n: usize, p: f64) -> Option<((usize, usize), (usize, usize), f64)> {
    let i = ((p * (n * n) as f64 - 0.5) as i64).clamp(0, (n * n) as i64 - 1) as usize;
    if i < n * n - 1 {
        let p0 = (i + 1) as f64 / (n * n) as f64; // どれだけ採用するか
        Some((get_pos(n, i), get_pos(n, i + 1), p0))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert_eq!(get_pos(20, 0), (0, 0));
        assert_eq!(get_pos(20, 19), (0, 19));
        assert_eq!(get_pos(20, 20), (1, 19));
        assert_eq!(get_pos(20, 39), (1, 0));
        assert_eq!(get_pos(20, 40), (2, 0));
        assert_eq!(get_pos(20, 399), (19, 0));
    }
}

// 仕切り移動あり - 絵具2回
fn update_candidates_4_2(
    n: usize,
    d: usize,
    cmy_own: &[(f64, f64, f64)],
    (c_t, m_t, y_t): (f64, f64, f64),
    v: &mut Vec<(f64, Vec<Vec<usize>>)>,
) {
    let k = 2.0;
    let pallete_len = cmy_own.len();
    for i0 in 0..(pallete_len - 1) {
        let (c0, m0, y0) = cmy_own[i0];
        for i1 in (i0 + 1)..pallete_len {
            let (c1, m1, y1) = cmy_own[i1];

            // 2色の絵具を内分して目標色に近づける
            if c0 == c1 && m0 == m1 && y0 == y1 {
                continue; // 同じ絵の具は使わない
            }

            let c01 = c1 - c0;
            let m01 = m1 - m0;
            let y01 = y1 - y0;
            let len01 = (c01.powi(2) + m01.powi(2) + y01.powi(2)).sqrt();
            let (c01n, m01n, y01n) = (c01 / len01, m01 / len01, y01 / len01);

            let c0t = c_t - c0;
            let m0t = m_t - m0;
            let y0t = y_t - y0;
            let inner_product = c0t * c01n + m0t * m01n + y0t * y01n;
            if inner_product < 0.0 || len01 < inner_product {
                continue; // 目標色が線分上にない
            }

            let p1 = inner_product / len01;
            let p0 = 1.0 - p1;

            // 操作のために並び替える
            let mut v0 = vec![(i0, p0, c0, m0, y0), (i1, p1, c1, m1, y1)];
            v0.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

            let ((i0, p0, c0, m0, y0), (i1, p1, c1, m1, y1)) = (v0[0], v0[1]);
            if p0 <= 0.0 {
                continue;
            }

            let mut ops = Vec::with_capacity(7);
            let mut g = 0.0; // パレット内に残っている絵の具量
            let (mut c, mut m, mut y) = (0.0, 0.0, 0.0); // パレット内の各色の絵の具量

            // 一番少ない絵の具を使う
            let p = p0 / p1;
            if let Some((pos0, pos1, px)) = get_divider_pos(n, p) {
                ops.push(vec![1, 0, 0, i0]); // 絵の具を1グラム出す
                ops.push(vec![4, pos0.0, pos0.1, pos1.0, pos1.1]); // 仕切りを出す
                ops.push(vec![3, n - 1, 0]); // 廃棄する
                ops.push(vec![4, pos0.0, pos0.1, pos1.0, pos1.1]); // 仕切りを戻す
                g = (g + 1.0) * px;
                c = (c + c0) * px;
                m = (m + m0) * px;
                y = (y + y0) * px;
            } else {
                ops.push(vec![1, 0, 0, i0]); // 絵の具を1グラム出す
                g += 1.0;
                c += c0;
                m += m0;
                y += y0;
            }

            // 次の絵の具を使う
            ops.push(vec![1, 0, 0, i1]); // 絵の具を1グラム出す
            ops.push(vec![2, 0, 0]); // 画伯に渡す
            g += 1.0;
            c += c1;
            m += m1;
            y += y1;
            (c, m, y) = (c / g, m / g, y / g);

            ops.push(vec![3, 0, 0]); // 廃棄する

            let dist2 = (c - c_t).powi(2) + (m - m_t).powi(2) + (y - y_t).powi(2);
            let score = dist2.sqrt() * K as f64 + (k - 1.0) * d as f64;
            let j = ops.len();
            if v[j].0 > score {
                v[j] = (score, ops);
            }
        }
    }
}

// 積
fn multiply(k: f64, (c, m, y): (f64, f64, f64)) -> (f64, f64, f64) {
    (k * c, k * m, k * y)
}

// 内積
fn innter_product((c01, m01, y01): (f64, f64, f64), (c02, m02, y02): (f64, f64, f64)) -> f64 {
    c01 * c02 + m01 * m02 + y01 * y02
}

// 外積
fn outer_product(
    (c01, m01, y01): (f64, f64, f64),
    (c02, m02, y02): (f64, f64, f64),
) -> (f64, f64, f64) {
    (
        m01 * y02 - m02 * y01,
        y01 * c02 - y02 * c01,
        c01 * m02 - c02 * m01,
    )
}

fn calc_dist((c, m, y): (f64, f64, f64)) -> f64 {
    (c * c + m * m + y * y).sqrt()
}

// 仕切り移動あり - 絵具3回
fn update_candidates_4_3(
    n: usize,
    d: usize,
    cmy_own: &[(f64, f64, f64)],
    (c_t, m_t, y_t): (f64, f64, f64),
    v: &mut Vec<(f64, Vec<Vec<usize>>)>,
) {
    let k = 3.0;
    let pallete_len = cmy_own.len();
    for i0 in 0..(pallete_len - 2) {
        let (c0, m0, y0) = cmy_own[i0];
        for i1 in (i0 + 1)..(pallete_len - 1) {
            let (c1, m1, y1) = cmy_own[i1];
            if c0 == c1 && m0 == m1 && y0 == y1 {
                continue; // 同じ絵の具は使わない
            }
            for i2 in (i1 + 1)..pallete_len {
                let (c2, m2, y2) = cmy_own[i2];
                if (c0 == c2 && m0 == m2 && y0 == y2) || (c1 == c2 && m1 == m2 && y1 == y2) {
                    continue; // 同じ絵の具は使わない
                }

                // 3色の絵具を内分して目標色に近づける
                // まず目標色を3つの絵の具から作られる平面上に投影する
                let (c01, m01, y01) = (c1 - c0, m1 - m0, y1 - y0);
                let (c12, m12, y12) = (c2 - c1, m2 - m1, y2 - y1);
                let (c20, m20, y20) = (c0 - c2, m0 - m2, y0 - y2);

                let outer012 = outer_product((c01, m01, y01), (-c20, -m20, -y20));
                let len = calc_dist(outer012);
                if len <= 0.0 {
                    continue;
                }
                let outer012n = multiply(1.0 / len, outer012); // 正規化した法線

                let (c0t, m0t, y0t) = (c_t - c0, m_t - m0, y_t - y0);
                let inner01t = innter_product((c0t, m0t, y0t), outer012n); // 内積成分を消したい
                let (c0t, m0t, y0t) = (
                    c0t - inner01t * outer012n.0,
                    m0t - inner01t * outer012n.1,
                    y0t - inner01t * outer012n.2,
                );
                let (ct, mt, yt) = (c0 + c0t, m0 + m0t, y0 + y0t); // 目標色

                let (c1t, m1t, y1t) = (ct - c1, mt - m1, yt - y1);
                let (c2t, m2t, y2t) = (ct - c2, mt - m2, yt - y2);

                // 目標色が外側になっていないかを確認する
                let outer01t = outer_product((c01, m01, y01), (c0t, m0t, y0t));
                let outer12t = outer_product((c12, m12, y12), (c1t, m1t, y1t));
                let outer20t = outer_product((c20, m20, y20), (c2t, m2t, y2t));

                if innter_product(outer01t, outer12t) < 0.0
                    || innter_product(outer12t, outer20t) < 0.0
                {
                    continue; // 目標色が外側にある
                }

                let (p0, p1, p2) = (
                    calc_dist(outer12t),
                    calc_dist(outer20t),
                    calc_dist(outer01t),
                );
                let p_sum = p0 + p1 + p2;
                let (p0, p1, p2) = (p0 / p_sum, p1 / p_sum, p2 / p_sum);

                // 操作のために並び替える
                let mut v0 = vec![
                    (i0, p0, c0, m0, y0),
                    (i1, p1, c1, m1, y1),
                    (i2, p2, c2, m2, y2),
                ];
                v0.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

                let ((i0, p0, c0, m0, y0), (i1, p1, c1, m1, y1), (i2, p2, c2, m2, y2)) =
                    (v0[0], v0[1], v0[2]);
                if p0 <= 0.0 {
                    continue;
                }

                // ふつうに絵の具を使う
                {
                    let mut ops = Vec::with_capacity(13);
                    let mut g = 0.0; // パレット内に残っている絵の具量
                    let (mut c, mut m, mut y) = (0.0, 0.0, 0.0); // パレット内の各色の絵の具量

                    // 一番少ない絵の具を使う
                    let p = p0 / p1;
                    if let Some((pos0, pos1, px)) = get_divider_pos(n, p) {
                        ops.push(vec![1, 0, 0, i0]); // 絵の具を1グラム出す
                        ops.push(vec![4, pos0.0, pos0.1, pos1.0, pos1.1]); // 仕切りを出す
                        ops.push(vec![3, n - 1, 0]); // 廃棄する
                        ops.push(vec![4, pos0.0, pos0.1, pos1.0, pos1.1]); // 仕切りを戻す
                        g = (g + 1.0) * px;
                        c = (c + c0) * px;
                        m = (m + m0) * px;
                        y = (y + y0) * px;
                    } else {
                        ops.push(vec![1, 0, 0, i0]); // 絵の具を1グラム出す
                        g += 1.0;
                        c += c0;
                        m += m0;
                        y += y0;
                    }

                    // 次の絵の具を使う
                    let p = p1 / p2;
                    if let Some((pos0, pos1, px)) = get_divider_pos(n, p) {
                        ops.push(vec![1, 0, 0, i1]); // 絵の具を1グラム出す
                        ops.push(vec![4, pos0.0, pos0.1, pos1.0, pos1.1]); // 仕切りを出す
                        let n3 = ((g + 1.0) * (1.0 - px) - TOL_ZERO).ceil() as usize;
                        for _ in 0..n3 {
                            ops.push(vec![3, n - 1, 0]); // 廃棄する
                        }
                        ops.push(vec![4, pos0.0, pos0.1, pos1.0, pos1.1]); // 仕切りを戻す
                        g = (g + 1.0) * px;
                        c = (c + c1) * px;
                        m = (m + m1) * px;
                        y = (y + y1) * px;
                    } else {
                        ops.push(vec![1, 0, 0, i1]); // 絵の具を1グラム出す
                        g += 1.0;
                        c += c1;
                        m += m1;
                        y += y1;
                    }

                    // 次の絵の具を使う
                    ops.push(vec![1, 0, 0, i2]); // 絵の具を1グラム出す
                    ops.push(vec![2, 0, 0]); // 画伯に渡す
                    g += 1.0;
                    c += c2;
                    m += m2;
                    y += y2;
                    (c, m, y) = (c / g, m / g, y / g);

                    g -= 1.0;
                    while g > TOL_ZERO {
                        ops.push(vec![3, 0, 0]); // 廃棄する
                        g -= 1.0;
                    }

                    let dist2 = (c - c_t).powi(2) + (m - m_t).powi(2) + (y - y_t).powi(2);
                    let score = dist2.sqrt() * K as f64 + (k - 1.0) * d as f64;
                    let j = ops.len();
                    if v[j].0 > score {
                        v[j] = (score, ops);
                    }
                }

                // 最初の絵の具2つは仕切りを使わない
                {
                    let mut ops = Vec::with_capacity(10);
                    let mut g = 0.0; // パレット内に残っている絵の具量
                    let (mut c, mut m, mut y) = (0.0, 0.0, 0.0); // パレット内の各色の絵の具量

                    // 絵の具を2つ使う
                    let p = ((p0 + p1) / 2.0) / p2;
                    if let Some((pos0, pos1, px)) = get_divider_pos(n, p) {
                        ops.push(vec![1, 0, 0, i0]); // 絵の具を1グラム出す
                        ops.push(vec![1, 0, 0, i1]); // 絵の具を1グラム出す
                        ops.push(vec![4, pos0.0, pos0.1, pos1.0, pos1.1]); // 仕切りを出す
                        let n3 = ((g + 2.0) * (1.0 - px) - TOL_ZERO).ceil() as usize;
                        for _ in 0..n3 {
                            ops.push(vec![3, n - 1, 0]); // 廃棄する
                        }
                        ops.push(vec![4, pos0.0, pos0.1, pos1.0, pos1.1]); // 仕切りを戻す
                        g = (g + 2.0) * px;
                        c = (c + c0 + c1) * px;
                        m = (m + m0 + m1) * px;
                        y = (y + y0 + y1) * px;
                    } else {
                        ops.push(vec![1, 0, 0, i0]); // 絵の具を1グラム出す
                        ops.push(vec![1, 0, 0, i1]); // 絵の具を1グラム出す
                        g += 2.0;
                        c += c0 + c1;
                        m += m0 + m1;
                        y += y0 + y1;
                    }

                    // 次の絵の具を使う
                    ops.push(vec![1, 0, 0, i2]); // 絵の具を1グラム出す
                    ops.push(vec![2, 0, 0]); // 画伯に渡す
                    g += 1.0;
                    c += c2;
                    m += m2;
                    y += y2;
                    (c, m, y) = (c / g, m / g, y / g);

                    g -= 1.0;
                    while g > TOL_ZERO {
                        ops.push(vec![3, 0, 0]); // 廃棄する
                        g -= 1.0;
                    }

                    let dist2 = (c - c_t).powi(2) + (m - m_t).powi(2) + (y - y_t).powi(2);
                    let score = dist2.sqrt() * K as f64 + (k - 1.0) * d as f64;
                    let j = ops.len();
                    if v[j].0 > score {
                        v[j] = (score, ops);
                    }
                }

                // 最後の絵の具2つは仕切りを使わない
                {
                    let mut ops = Vec::with_capacity(10);
                    let mut g = 0.0; // パレット内に残っている絵の具量
                    let (mut c, mut m, mut y) = (0.0, 0.0, 0.0); // パレット内の各色の絵の具量

                    // 一番少ない絵の具を使う
                    let p = 2.0 * p0 / (1.0 - p0);
                    if let Some((pos0, pos1, px)) = get_divider_pos(n, p) {
                        ops.push(vec![1, 0, 0, i0]); // 絵の具を1グラム出す
                        ops.push(vec![4, pos0.0, pos0.1, pos1.0, pos1.1]); // 仕切りを出す
                        ops.push(vec![3, n - 1, 0]); // 廃棄する
                        ops.push(vec![4, pos0.0, pos0.1, pos1.0, pos1.1]); // 仕切りを戻す
                        g = (g + 1.0) * px;
                        c = (c + c0) * px;
                        m = (m + m0) * px;
                        y = (y + y0) * px;
                    } else {
                        ops.push(vec![1, 0, 0, i0]); // 絵の具を1グラム出す
                        g += 1.0;
                        c += c0;
                        m += m0;
                        y += y0;
                    }

                    // 次の絵の具を使う
                    ops.push(vec![1, 0, 0, i1]); // 絵の具を1グラム出す
                    ops.push(vec![1, 0, 0, i2]); // 絵の具を1グラム出す
                    ops.push(vec![2, 0, 0]); // 画伯に渡す
                    g += 2.0;
                    c += c1 + c2;
                    m += m1 + m2;
                    y += y1 + y2;
                    (c, m, y) = (c / g, m / g, y / g);

                    g -= 1.0;
                    while g > TOL_ZERO {
                        ops.push(vec![3, 0, 0]); // 廃棄する
                        g -= 1.0;
                    }

                    let dist2 = (c - c_t).powi(2) + (m - m_t).powi(2) + (y - y_t).powi(2);
                    let score = dist2.sqrt() * K as f64 + (k - 1.0) * d as f64;
                    let j = ops.len();
                    if v[j].0 > score {
                        v[j] = (score, ops);
                    }
                }
            }
        }
    }
}

// 仕切り移動あり - 絵具4回
fn update_candidates_4_4(
    n: usize,
    d: usize,
    cmy_own: &[(f64, f64, f64)],
    (c_t, m_t, y_t): (f64, f64, f64),
    v: &mut Vec<(f64, Vec<Vec<usize>>)>,
) {
    let k = 4.0;
    let pallete_len = cmy_own.len();
    for i0 in 0..(pallete_len - 3) {
        let (c0, m0, y0) = cmy_own[i0];
        for i1 in (i0 + 1)..(pallete_len - 2) {
            let (c1, m1, y1) = cmy_own[i1];
            if c0 == c1 && m0 == m1 && y0 == y1 {
                continue; // 同じ絵の具は使わない
            }
            for i2 in (i1 + 1)..(pallete_len - 1) {
                let (c2, m2, y2) = cmy_own[i2];
                if (c0 == c2 && m0 == m2 && y0 == y2) || (c1 == c2 && m1 == m2 && y1 == y2) {
                    continue; // 同じ絵の具は使わない
                }
                for i3 in (i2 + 1)..pallete_len {
                    let (c3, m3, y3) = cmy_own[i3];
                    if (c0 == c3 && m0 == m3 && y0 == y3)
                        || (c1 == c3 && m1 == m3 && y1 == y3)
                        || (c2 == c3 && m2 == m3 && y2 == y3)
                    {
                        continue; // 同じ絵の具は使わない
                    }

                    // 4色の絵具を内分して目標色に近づける

                    // 逆行列を求める
                    let a = [
                        [c0 - c3, c1 - c3, c2 - c3],
                        [m0 - m3, m1 - m3, m2 - m3],
                        [y0 - y3, y1 - y3, y2 - y3],
                    ];
                    let len = a[0][0] * a[1][1] * a[2][2]
                        + a[0][1] * a[1][2] * a[2][0]
                        + a[0][2] * a[1][0] * a[2][1]
                        - a[0][2] * a[1][1] * a[2][0]
                        - a[0][1] * a[1][0] * a[2][2]
                        - a[0][0] * a[1][2] * a[2][1];
                    if len == 0.0 {
                        continue;
                    }
                    let len = 1.0 / len;
                    let a = [
                        [
                            a[1][1] * a[2][2] - a[1][2] * a[2][1],
                            a[0][2] * a[2][1] - a[0][1] * a[2][2],
                            a[0][1] * a[1][2] - a[0][2] * a[1][1],
                        ],
                        [
                            a[1][2] * a[2][0] - a[1][0] * a[2][2],
                            a[0][0] * a[2][2] - a[0][2] * a[2][0],
                            a[0][2] * a[1][0] - a[0][0] * a[1][2],
                        ],
                        [
                            a[1][0] * a[2][1] - a[1][1] * a[2][0],
                            a[0][1] * a[2][0] - a[0][0] * a[2][1],
                            a[0][0] * a[1][1] - a[0][1] * a[1][0],
                        ],
                    ];
                    let a = [
                        [a[0][0] * len, a[0][1] * len, a[0][2] * len],
                        [a[1][0] * len, a[1][1] * len, a[1][2] * len],
                        [a[2][0] * len, a[2][1] * len, a[2][2] * len],
                    ];

                    let (c3t, m3t, y3t) = (c_t - c3, m_t - m3, y_t - y3);
                    let p0 = a[0][0] * c3t + a[0][1] * m3t + a[0][2] * y3t;
                    let p1 = a[1][0] * c3t + a[1][1] * m3t + a[1][2] * y3t;
                    let p2 = a[2][0] * c3t + a[2][1] * m3t + a[2][2] * y3t;
                    let p3 = 1.0 - p0 - p1 - p2;
                    if (p0 < 0.0 || p1 < 0.0 || p2 < 0.0 || p3 < 0.0)
                        || (p0 > 1.0 || p1 > 1.0 || p2 > 1.0 || p3 > 1.0)
                    {
                        continue; // 目標色が外側にある
                    }

                    // 操作のために並び替える
                    let mut v0 = vec![
                        (i0, p0, c0, m0, y0),
                        (i1, p1, c1, m1, y1),
                        (i2, p2, c2, m2, y2),
                        (i3, p3, c3, m3, y3),
                    ];
                    v0.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

                    let (
                        (i0, p0, c0, m0, y0),
                        (i1, p1, c1, m1, y1),
                        (i2, p2, c2, m2, y2),
                        (i3, p3, c3, m3, y3),
                    ) = (v0[0], v0[1], v0[2], v0[3]);
                    if p0 <= 0.0 {
                        continue;
                    }

                    // ふつうに絵の具を使う
                    {
                        let mut ops = Vec::with_capacity(18);
                        let mut g = 0.0; // パレット内に残っている絵の具量
                        let (mut c, mut m, mut y) = (0.0, 0.0, 0.0); // パレット内の各色の絵の具量

                        // 一番少ない絵の具を使う
                        let p = p0 / p1;
                        if let Some((pos0, pos1, px)) = get_divider_pos(n, p) {
                            ops.push(vec![1, 0, 0, i0]); // 絵の具を1グラム出す
                            ops.push(vec![4, pos0.0, pos0.1, pos1.0, pos1.1]); // 仕切りを出す
                            ops.push(vec![3, n - 1, 0]); // 廃棄する
                            ops.push(vec![4, pos0.0, pos0.1, pos1.0, pos1.1]); // 仕切りを戻す
                            g = (g + 1.0) * px;
                            c = (c + c0) * px;
                            m = (m + m0) * px;
                            y = (y + y0) * px;
                        } else {
                            ops.push(vec![1, 0, 0, i0]); // 絵の具を1グラム出す
                            g += 1.0;
                            c += c0;
                            m += m0;
                            y += y0;
                        }

                        // 次の絵の具を使う
                        let p = p1 / p2;
                        if let Some((pos0, pos1, px)) = get_divider_pos(n, p) {
                            ops.push(vec![1, 0, 0, i1]); // 絵の具を1グラム出す
                            ops.push(vec![4, pos0.0, pos0.1, pos1.0, pos1.1]); // 仕切りを出す
                            let n3 = ((g + 1.0) * (1.0 - px) - TOL_ZERO).ceil() as usize;
                            for _ in 0..n3 {
                                ops.push(vec![3, n - 1, 0]); // 廃棄する
                            }
                            ops.push(vec![4, pos0.0, pos0.1, pos1.0, pos1.1]); // 仕切りを戻す
                            g = (g + 1.0) * px;
                            c = (c + c1) * px;
                            m = (m + m1) * px;
                            y = (y + y1) * px;
                        } else {
                            ops.push(vec![1, 0, 0, i1]); // 絵の具を1グラム出す
                            g += 1.0;
                            c += c1;
                            m += m1;
                            y += y1;
                        }

                        // 次の絵の具を使う
                        let p = p2 / p3;
                        if let Some((pos0, pos1, px)) = get_divider_pos(n, p) {
                            ops.push(vec![1, 0, 0, i2]); // 絵の具を1グラム出す
                            ops.push(vec![4, pos0.0, pos0.1, pos1.0, pos1.1]); // 仕切りを出す
                            let n3 = ((g + 1.0) * (1.0 - px) - TOL_ZERO).ceil() as usize;
                            for _ in 0..n3 {
                                ops.push(vec![3, n - 1, 0]); // 廃棄する
                            }
                            ops.push(vec![4, pos0.0, pos0.1, pos1.0, pos1.1]); // 仕切りを戻す
                            g = (g + 1.0) * px;
                            c = (c + c2) * px;
                            m = (m + m2) * px;
                            y = (y + y2) * px;
                        } else {
                            ops.push(vec![1, 0, 0, i2]); // 絵の具を1グラム出す
                            g += 1.0;
                            c += c2;
                            m += m2;
                            y += y2;
                        }

                        // 次の絵の具を使う
                        ops.push(vec![1, 0, 0, i3]); // 絵の具を1グラム出す
                        ops.push(vec![2, 0, 0]); // 画伯に渡す
                        c += c3;
                        m += m3;
                        y += y3;
                        g += 1.0;
                        (c, m, y) = (c / g, m / g, y / g);

                        g -= 1.0;
                        while g > TOL_ZERO {
                            ops.push(vec![3, 0, 0]); // 廃棄する
                            g -= 1.0;
                        }

                        let dist2 = (c - c_t).powi(2) + (m - m_t).powi(2) + (y - y_t).powi(2);
                        let score = dist2.sqrt() * K as f64 + (k - 1.0) * d as f64;
                        let j = ops.len();
                        if v[j].0 > score {
                            v[j] = (score, ops);
                        }
                    }

                    // 最初の絵の具2つは仕切りを使わない
                    {
                        let mut ops = Vec::with_capacity(15);
                        let mut g = 0.0; // パレット内に残っている絵の具量
                        let (mut c, mut m, mut y) = (0.0, 0.0, 0.0); // パレット内の各色の絵の具量

                        // 一番少ない絵の具を使う
                        let p = ((p0 + p1) / 2.0) / p2;
                        if let Some((pos0, pos1, px)) = get_divider_pos(n, p) {
                            ops.push(vec![1, 0, 0, i0]); // 絵の具を1グラム出す
                            ops.push(vec![1, 0, 0, i1]); // 絵の具を1グラム出す
                            ops.push(vec![4, pos0.0, pos0.1, pos1.0, pos1.1]); // 仕切りを出す
                            let n3 = ((g + 2.0) * (1.0 - px) - TOL_ZERO).ceil() as usize;
                            for _ in 0..n3 {
                                ops.push(vec![3, n - 1, 0]); // 廃棄する
                            }
                            ops.push(vec![4, pos0.0, pos0.1, pos1.0, pos1.1]); // 仕切りを戻す
                            g = (g + 2.0) * px;
                            c = (c + c0 + c1) * px;
                            m = (m + m0 + m1) * px;
                            y = (y + y0 + y1) * px;
                        } else {
                            ops.push(vec![1, 0, 0, i0]); // 絵の具を1グラム出す
                            ops.push(vec![1, 0, 0, i1]); // 絵の具を1グラム出す
                            g += 2.0;
                            c += c0 + c1;
                            m += m0 + m1;
                            y += y0 + y1;
                        }

                        // 次の絵の具を使う
                        let p = p2 / p3;
                        if let Some((pos0, pos1, px)) = get_divider_pos(n, p) {
                            ops.push(vec![1, 0, 0, i2]); // 絵の具を1グラム出す
                            ops.push(vec![4, pos0.0, pos0.1, pos1.0, pos1.1]); // 仕切りを出す
                            let n3 = ((g + 1.0) * (1.0 - px) - TOL_ZERO).ceil() as usize;
                            for _ in 0..n3 {
                                ops.push(vec![3, n - 1, 0]); // 廃棄する
                            }
                            ops.push(vec![4, pos0.0, pos0.1, pos1.0, pos1.1]); // 仕切りを戻す
                            g = (g + 1.0) * px;
                            c = (c + c2) * px;
                            m = (m + m2) * px;
                            y = (y + y2) * px;
                        } else {
                            ops.push(vec![1, 0, 0, i2]); // 絵の具を1グラム出す
                            g += 1.0;
                            c += c2;
                            m += m2;
                            y += y2;
                        }

                        // 次の絵の具を使う
                        ops.push(vec![1, 0, 0, i3]); // 絵の具を1グラム出す
                        ops.push(vec![2, 0, 0]); // 画伯に渡す
                        c += c3;
                        m += m3;
                        y += y3;
                        g += 1.0;
                        (c, m, y) = (c / g, m / g, y / g);

                        g -= 1.0;
                        while g > TOL_ZERO {
                            ops.push(vec![3, 0, 0]); // 廃棄する
                            g -= 1.0;
                        }

                        let dist2 = (c - c_t).powi(2) + (m - m_t).powi(2) + (y - y_t).powi(2);
                        let score = dist2.sqrt() * K as f64 + (k - 1.0) * d as f64;
                        let j = ops.len();
                        if v[j].0 > score {
                            v[j] = (score, ops);
                        }
                    }

                    // 最初の絵の具3つは仕切りを使わない
                    {
                        let mut ops = Vec::with_capacity(12);
                        let mut g = 0.0; // パレット内に残っている絵の具量
                        let (mut c, mut m, mut y) = (0.0, 0.0, 0.0); // パレット内の各色の絵の具量

                        // 一番少ない絵の具を使う
                        let p = ((p0 + p1 + p2) / 3.0) / p3;
                        if let Some((pos0, pos1, px)) = get_divider_pos(n, p) {
                            ops.push(vec![1, 0, 0, i0]); // 絵の具を1グラム出す
                            ops.push(vec![1, 0, 0, i1]); // 絵の具を1グラム出す
                            ops.push(vec![1, 0, 0, i2]); // 絵の具を1グラム出す
                            ops.push(vec![4, pos0.0, pos0.1, pos1.0, pos1.1]); // 仕切りを出す
                            let n3 = ((g + 3.0) * (1.0 - px) - TOL_ZERO).ceil() as usize;
                            for _ in 0..n3 {
                                ops.push(vec![3, n - 1, 0]); // 廃棄する
                            }
                            ops.push(vec![4, pos0.0, pos0.1, pos1.0, pos1.1]); // 仕切りを戻す
                            g = (g + 3.0) * px;
                            c = (c + c0 + c1 + c2) * px;
                            m = (m + m0 + m1 + m2) * px;
                            y = (y + y0 + y1 + y2) * px;
                        } else {
                            ops.push(vec![1, 0, 0, i0]); // 絵の具を1グラム出す
                            ops.push(vec![1, 0, 0, i1]); // 絵の具を1グラム出す
                            ops.push(vec![1, 0, 0, i2]); // 絵の具を1グラム出す
                            g += 3.0;
                            c += c0 + c1 + c2;
                            m += m0 + m1 + m2;
                            y += y0 + y1 + y2;
                        }

                        // 次の絵の具を使う
                        ops.push(vec![1, 0, 0, i3]); // 絵の具を1グラム出す
                        ops.push(vec![2, 0, 0]); // 画伯に渡す
                        c += c3;
                        m += m3;
                        y += y3;
                        g += 1.0;
                        (c, m, y) = (c / g, m / g, y / g);

                        g -= 1.0;
                        while g > TOL_ZERO {
                            ops.push(vec![3, 0, 0]); // 廃棄する
                            g -= 1.0;
                        }

                        let dist2 = (c - c_t).powi(2) + (m - m_t).powi(2) + (y - y_t).powi(2);
                        let score = dist2.sqrt() * K as f64 + (k - 1.0) * d as f64;
                        let j = ops.len();
                        if v[j].0 > score {
                            v[j] = (score, ops);
                        }
                    }

                    // 途中の2つは仕切りを使わない
                    {
                        let mut ops = Vec::with_capacity(15);
                        let mut g = 0.0; // パレット内に残っている絵の具量
                        let (mut c, mut m, mut y) = (0.0, 0.0, 0.0); // パレット内の各色の絵の具量

                        // 一番少ない絵の具を使う
                        let p = p0 / p1;
                        if let Some((pos0, pos1, px)) = get_divider_pos(n, p) {
                            ops.push(vec![1, 0, 0, i0]); // 絵の具を1グラム出す
                            ops.push(vec![4, pos0.0, pos0.1, pos1.0, pos1.1]); // 仕切りを出す
                            ops.push(vec![3, n - 1, 0]); // 廃棄する
                            ops.push(vec![4, pos0.0, pos0.1, pos1.0, pos1.1]); // 仕切りを戻す
                            g = (g + 1.0) * px;
                            c = (c + c0) * px;
                            m = (m + m0) * px;
                            y = (y + y0) * px;
                        } else {
                            ops.push(vec![1, 0, 0, i0]); // 絵の具を1グラム出す
                            g += 1.0;
                            c += c0;
                            m += m0;
                            y += y0;
                        }

                        // 次の絵の具を使う
                        let p = ((p1 + p2) / 2.0) / p3;
                        if let Some((pos0, pos1, px)) = get_divider_pos(n, p) {
                            ops.push(vec![1, 0, 0, i1]); // 絵の具を1グラム出す
                            ops.push(vec![1, 0, 0, i2]); // 絵の具を1グラム出す
                            ops.push(vec![4, pos0.0, pos0.1, pos1.0, pos1.1]); // 仕切りを出す
                            let n3 = ((g + 2.0) * (1.0 - px) - TOL_ZERO).ceil() as usize;
                            for _ in 0..n3 {
                                ops.push(vec![3, n - 1, 0]); // 廃棄する
                            }
                            ops.push(vec![4, pos0.0, pos0.1, pos1.0, pos1.1]); // 仕切りを戻す
                            g = (g + 2.0) * px;
                            c = (c + c1 + c2) * px;
                            m = (m + m1 + m2) * px;
                            y = (y + y1 + y2) * px;
                        } else {
                            ops.push(vec![1, 0, 0, i1]); // 絵の具を1グラム出す
                            ops.push(vec![1, 0, 0, i2]); // 絵の具を1グラム出す
                            g += 2.0;
                            c += c1 + c2;
                            m += m1 + m2;
                            y += y1 + y2;
                        }

                        // 次の絵の具を使う
                        ops.push(vec![1, 0, 0, i3]); // 絵の具を1グラム出す
                        ops.push(vec![2, 0, 0]); // 画伯に渡す
                        c += c3;
                        m += m3;
                        y += y3;
                        g += 1.0;
                        (c, m, y) = (c / g, m / g, y / g);

                        g -= 1.0;
                        while g > TOL_ZERO {
                            ops.push(vec![3, 0, 0]); // 廃棄する
                            g -= 1.0;
                        }

                        let dist2 = (c - c_t).powi(2) + (m - m_t).powi(2) + (y - y_t).powi(2);
                        let score = dist2.sqrt() * K as f64 + (k - 1.0) * d as f64;
                        let j = ops.len();
                        if v[j].0 > score {
                            v[j] = (score, ops);
                        }
                    }

                    // 最後の2つは仕切りを使わない
                    {
                        let mut ops = Vec::with_capacity(15);
                        let mut g = 0.0; // パレット内に残っている絵の具量
                        let (mut c, mut m, mut y) = (0.0, 0.0, 0.0); // パレット内の各色の絵の具量

                        // 一番少ない絵の具を使う
                        let p = p0 / p1;
                        if let Some((pos0, pos1, px)) = get_divider_pos(n, p) {
                            ops.push(vec![1, 0, 0, i0]); // 絵の具を1グラム出す
                            ops.push(vec![4, pos0.0, pos0.1, pos1.0, pos1.1]); // 仕切りを出す
                            ops.push(vec![3, n - 1, 0]); // 廃棄する
                            ops.push(vec![4, pos0.0, pos0.1, pos1.0, pos1.1]); // 仕切りを戻す
                            g = (g + 1.0) * px;
                            c = (c + c0) * px;
                            m = (m + m0) * px;
                            y = (y + y0) * px;
                        } else {
                            ops.push(vec![1, 0, 0, i0]); // 絵の具を1グラム出す
                            g += 1.0;
                            c += c0;
                            m += m0;
                            y += y0;
                        }

                        // 次の絵の具を使う
                        let p = p1 / ((p2 + p3) / 2.0);
                        if let Some((pos0, pos1, px)) = get_divider_pos(n, p) {
                            ops.push(vec![1, 0, 0, i1]); // 絵の具を1グラム出す
                            ops.push(vec![4, pos0.0, pos0.1, pos1.0, pos1.1]); // 仕切りを出す
                            let n3 = ((g + 1.0) * (1.0 - px) - TOL_ZERO).ceil() as usize;
                            for _ in 0..n3 {
                                ops.push(vec![3, n - 1, 0]); // 廃棄する
                            }
                            ops.push(vec![4, pos0.0, pos0.1, pos1.0, pos1.1]); // 仕切りを戻す
                            g = (g + 1.0) * px;
                            c = (c + c1) * px;
                            m = (m + m1) * px;
                            y = (y + y1) * px;
                        } else {
                            ops.push(vec![1, 0, 0, i1]); // 絵の具を1グラム出す
                            g += 1.0;
                            c += c1;
                            m += m1;
                            y += y1;
                        }

                        // 次の絵の具を使う
                        ops.push(vec![1, 0, 0, i2]); // 絵の具を1グラム出す
                        ops.push(vec![1, 0, 0, i3]); // 絵の具を1グラム出す
                        ops.push(vec![2, 0, 0]); // 画伯に渡す
                        g += 2.0;
                        c += c2 + c3;
                        m += m2 + m3;
                        y += y2 + y3;
                        (c, m, y) = (c / g, m / g, y / g);

                        g -= 1.0;
                        while g > TOL_ZERO {
                            ops.push(vec![3, 0, 0]); // 廃棄する
                            g -= 1.0;
                        }

                        let dist2 = (c - c_t).powi(2) + (m - m_t).powi(2) + (y - y_t).powi(2);
                        let score = dist2.sqrt() * K as f64 + (k - 1.0) * d as f64;
                        let j = ops.len();
                        if v[j].0 > score {
                            v[j] = (score, ops);
                        }
                    }

                    // 最後の3つは仕切りを使わない
                    {
                        let mut ops = Vec::with_capacity(12);
                        let mut g = 0.0; // パレット内に残っている絵の具量
                        let (mut c, mut m, mut y) = (0.0, 0.0, 0.0); // パレット内の各色の絵の具量

                        // 一番少ない絵の具を使う
                        let p = p0 / ((p1 + p2 + p3) / 3.0);
                        if let Some((pos0, pos1, px)) = get_divider_pos(n, p) {
                            ops.push(vec![1, 0, 0, i0]); // 絵の具を1グラム出す
                            ops.push(vec![4, pos0.0, pos0.1, pos1.0, pos1.1]); // 仕切りを出す
                            ops.push(vec![3, n - 1, 0]); // 廃棄する
                            ops.push(vec![4, pos0.0, pos0.1, pos1.0, pos1.1]); // 仕切りを戻す
                            g = (g + 1.0) * px;
                            c = (c + c0) * px;
                            m = (m + m0) * px;
                            y = (y + y0) * px;
                        } else {
                            ops.push(vec![1, 0, 0, i0]); // 絵の具を1グラム出す
                            g += 1.0;
                            c += c0;
                            m += m0;
                            y += y0;
                        }

                        // 次の絵の具を使う
                        ops.push(vec![1, 0, 0, i1]); // 絵の具を1グラム出す
                        ops.push(vec![1, 0, 0, i2]); // 絵の具を1グラム出す
                        ops.push(vec![1, 0, 0, i3]); // 絵の具を1グラム出す
                        ops.push(vec![2, 0, 0]); // 画伯に渡す
                        g += 3.0;
                        c += c1 + c2 + c3;
                        m += m1 + m2 + m3;
                        y += y1 + y2 + y3;
                        (c, m, y) = (c / g, m / g, y / g);

                        g -= 1.0;
                        while g > TOL_ZERO {
                            ops.push(vec![3, 0, 0]); // 廃棄する
                            g -= 1.0;
                        }

                        let dist2 = (c - c_t).powi(2) + (m - m_t).powi(2) + (y - y_t).powi(2);
                        let score = dist2.sqrt() * K as f64 + (k - 1.0) * d as f64;
                        let j = ops.len();
                        if v[j].0 > score {
                            v[j] = (score, ops);
                        }
                    }

                    // 最初の2つと最後の2つは仕切りを使わない
                    {
                        let mut ops = Vec::with_capacity(12);
                        let mut g = 0.0; // パレット内に残っている絵の具量
                        let (mut c, mut m, mut y) = (0.0, 0.0, 0.0); // パレット内の各色の絵の具量

                        // 一番少ない絵の具を使う
                        let p = ((p0 + p1) / 2.0) / ((p2 + p3) / 2.0);
                        if let Some((pos0, pos1, px)) = get_divider_pos(n, p) {
                            ops.push(vec![1, 0, 0, i0]); // 絵の具を1グラム出す
                            ops.push(vec![1, 0, 0, i1]); // 絵の具を1グラム出す
                            ops.push(vec![4, pos0.0, pos0.1, pos1.0, pos1.1]); // 仕切りを出す
                            let n3 = ((g + 2.0) * (1.0 - px) - TOL_ZERO).ceil() as usize;
                            for _ in 0..n3 {
                                ops.push(vec![3, n - 1, 0]); // 廃棄する
                            }
                            ops.push(vec![4, pos0.0, pos0.1, pos1.0, pos1.1]); // 仕切りを戻す
                            g = (g + 2.0) * px;
                            c = (c + c0 + c1) * px;
                            m = (m + m0 + m1) * px;
                            y = (y + y0 + y1) * px;
                        } else {
                            ops.push(vec![1, 0, 0, i0]); // 絵の具を1グラム出す
                            ops.push(vec![1, 0, 0, i1]); // 絵の具を1グラム出す
                            g += 2.0;
                            c += c0 + c1;
                            m += m0 + m1;
                            y += y0 + y1;
                        }

                        // 次の絵の具を使う
                        ops.push(vec![1, 0, 0, i2]); // 絵の具を1グラム出す
                        ops.push(vec![1, 0, 0, i3]); // 絵の具を1グラム出す
                        ops.push(vec![2, 0, 0]); // 画伯に渡す
                        g += 2.0;
                        c += c2 + c3;
                        m += m2 + m3;
                        y += y2 + y3;
                        (c, m, y) = (c / g, m / g, y / g);

                        g -= 1.0;
                        while g > TOL_ZERO {
                            ops.push(vec![3, 0, 0]); // 廃棄する
                            g -= 1.0;
                        }

                        let dist2 = (c - c_t).powi(2) + (m - m_t).powi(2) + (y - y_t).powi(2);
                        let score = dist2.sqrt() * K as f64 + (k - 1.0) * d as f64;
                        let j = ops.len();
                        if v[j].0 > score {
                            v[j] = (score, ops);
                        }
                    }

                    // 仕切りを使わない
                    {
                        let ops = build_ops_123(&[i0, i1, i2, i3]);
                        let g = 4.0; // パレット内に残っている絵の具量
                        let c = (c0 + c1 + c2 + c3) / g;
                        let m = (m0 + m1 + m2 + m3) / g;
                        let y = (y0 + y1 + y2 + y3) / g;

                        let dist2 = (c - c_t).powi(2) + (m - m_t).powi(2) + (y - y_t).powi(2);
                        let score = dist2.sqrt() * K as f64 + (k - 1.0) * d as f64;
                        let j = ops.len();
                        if v[j].0 > score {
                            v[j] = (score, ops);
                        }
                    }
                }
            }
        }
    }
}

fn build_candidates(
    n: usize,
    d: usize,
    cmy_own: &[(f64, f64, f64)],
    cmy_t: (f64, f64, f64),
) -> Vec<(f64, Vec<Vec<usize>>)> {
    const N: usize = 20; // 1回渡すための最大手数
    let mut v = vec![(f64::MAX, vec![]); N];

    update_candidates_123_1(d, cmy_own, cmy_t, &mut v);
    update_candidates_123_2(d, cmy_own, cmy_t, &mut v);
    update_candidates_123_3(d, cmy_own, cmy_t, &mut v);
    // update_candidates_123_4(d, cmy_own, cmy_t, &mut v);

    update_candidates_4_2(n, d, cmy_own, cmy_t, &mut v);
    update_candidates_4_3(n, d, cmy_own, cmy_t, &mut v);
    update_candidates_4_4(n, d, cmy_own, cmy_t, &mut v);

    // 不要な選択肢を消す
    let mut results = vec![];
    for (score, v) in v {
        if score == f64::MAX {
            continue;
        }
        if let Some(&(last_score, _)) = results.last() {
            if score >= last_score {
                continue;
            }
        }
        results.push((score, v));
    }

    results
}

fn build_results(
    _n: usize,
    t: usize,
    _d: usize,
    candidates: &[Vec<(f64, Vec<Vec<usize>>)>],
) -> (f64, Vec<Vec<usize>>) {
    let h = candidates.len();

    let k = (K * K) as f64;
    let mut sum_score = 1.0;
    let mut sum_len = 0;
    let mut v = vec![0; h]; // 採用する位置
    let mut queue = BinaryHeap::new();
    for (i, cand) in candidates.iter().enumerate() {
        sum_len += cand[cand.len() - 1].1.len();
        sum_score += cand[cand.len() - 1].0;
        let j = cand.len() - 1;
        v[i] = j;
        if j > 0 {
            queue.push((
                ((cand[j].0 - cand[j - 1].0) * k / (cand[j].1.len() - cand[j - 1].1.len()) as f64)
                    as i64,
                i,
                j,
            ));
        }
    }

    while sum_len > t {
        let Some((_, i, j)) = queue.pop() else {
            unreachable!()
        }; // 一番スコアが上がらないものを除く
        let cand = &candidates[i];
        sum_len -= cand[j].1.len();
        sum_score -= cand[j].0;
        let j = j - 1;
        sum_len += cand[j].1.len();
        sum_score += cand[j].0;
        v[i] = j;
        if j > 0 {
            queue.push((
                ((cand[j].0 - cand[j - 1].0) * k / (cand[j].1.len() - cand[j - 1].1.len()) as f64)
                    as i64,
                i,
                j,
            ));
        }
    }

    let mut results = vec![];
    for (i, j) in v.iter().enumerate() {
        let mut v = candidates[i][*j].1.clone();
        results.append(&mut v);
    }

    (sum_score, results)
}

fn build_another_results(
    n: usize,
    t: usize,
    d: usize,
    cmy_own: &[(f64, f64, f64)],
    cmy_target: &[(f64, f64, f64)],
) -> (f64, Vec<Vec<usize>>) {
    let mut ops = Vec::with_capacity(cmy_target.len() * 4 + 1);
    let mut total_score = 1.0 + d as f64; // 最小数より1回多く絵の具を出す

    // 初手
    let mut i = 0;
    {
        let mut score1 = f64::MAX;

        let (ct, mt, yt) = cmy_target[0];
        let pallete_len = cmy_own.len();
        for i0 in 0..(pallete_len - 1) {
            let (c0, m0, y0) = cmy_own[i0];
            for i1 in (i0 + 1)..pallete_len {
                let (c1, m1, y1) = cmy_own[i1];

                // 2色の絵具を内分して目標色に近づける
                if c0 == c1 && m0 == m1 && y0 == y1 {
                    continue; // 同じ絵の具は使わない
                }

                let c01 = c1 - c0;
                let m01 = m1 - m0;
                let y01 = y1 - y0;
                let len01 = (c01.powi(2) + m01.powi(2) + y01.powi(2)).sqrt();
                let (c01n, m01n, y01n) = (c01 / len01, m01 / len01, y01 / len01);

                let c0t = ct - c0;
                let m0t = mt - m0;
                let y0t = yt - y0;
                let inner_product = c0t * c01n + m0t * m01n + y0t * y01n;
                if inner_product < 0.0 || len01 < inner_product {
                    continue; // 目標色が線分上にない
                }

                let p1 = inner_product / len01;
                let p0 = 1.0 - p1;

                // 操作のために並び替える
                let mut v0 = vec![(i0, p0, c0, m0, y0), (i1, p1, c1, m1, y1)];
                v0.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

                let ((i0, p0, c0, m0, y0), (_, p1, c1, m1, y1)) = (v0[0], v0[1]);
                if p0 <= 0.0 {
                    continue;
                }

                let (c2, m2, y2) = (c0 * p0 + c1 * p1, m0 * p0 + m1 * p1, y0 * p0 + y1 * p1);

                let dist2 = (c2 - ct).powi(2) + (m2 - mt).powi(2) + (y2 - yt).powi(2);
                let score2 = dist2.sqrt() * K as f64;
                if score1 > score2 {
                    score1 = score2;
                    i = i0; // 比率の少ない方を選ぶ
                }
            }
        }
    }

    let (mut c, mut m, mut y) = cmy_own[i];
    ops.push(vec![1, 0, 0, i]); // 絵の具を1グラム出す
    eprintln!("first: {} {} {} {}", i, c, m, y);

    // 2手目以降
    for (ct, mt, yt) in cmy_target {
        let mut i1 = 0usize; // 絵の具の番号
        let mut i2 = 0usize; // 前の絵の具用に仕切りをどこに入れるか。 0は使わない相当

        let (c0, m0, y0) = cmy_own[0];
        let (mut c1, mut m1, mut y1) = ((c + c0) / 2.0, (m + m0) / 2.0, (y + y0) / 2.0); // 一番良い位置
        let dist2 = (c1 - ct).powi(2) + (m1 - mt).powi(2) + (y1 - yt).powi(2);
        let mut score1 = dist2.sqrt() * K as f64; // 一番良いスコア

        for (i0, &(c0, m0, y0)) in cmy_own.iter().enumerate() {
            let (c2, m2, y2) = ((c + c0) / 2.0, (m + m0) / 2.0, (y + y0) / 2.0);
            let dist2 = (c2 - ct).powi(2) + (m2 - mt).powi(2) + (y2 - yt).powi(2);
            let score2 = dist2.sqrt() * K as f64;
            if score1 > score2 {
                score1 = score2;
                i1 = i0;
                i2 = 0;
                (c1, m1, y1) = (c2, m2, y2);
            }

            for i in 2..(n * n) {
                let p2 = i as f64 / (i + n * n) as f64; // 前の色をどれだけ使うか
                let p3 = 1.0 - p2;
                let (c2, m2, y2) = (c * p2 + c0 * p3, m * p2 + m0 * p3, y * p2 + y0 * p3);
                let dist2 = (c2 - ct).powi(2) + (m2 - mt).powi(2) + (y2 - yt).powi(2);
                let score2 = dist2.sqrt() * K as f64;
                if score1 > score2 {
                    score1 = score2;
                    i1 = i0;
                    i2 = i;
                    (c1, m1, y1) = (c2, m2, y2);
                }
            }
        }

        total_score += score1;
        let (c0, m0, y0) = cmy_own[i1];
        (c, m, y) = (c + c0 - c1, m + m0 - m1, y + y0 - y1);
        if i2 == 0 {
            ops.push(vec![1, 0, 0, i1]); // 絵の具を1グラム出す
            ops.push(vec![2, 0, 0]); // 画伯に渡す
        } else {
            let pos0 = get_pos(n, i2 - 1);
            let pos1 = get_pos(n, i2);
            ops.push(vec![4, pos0.0, pos0.1, pos1.0, pos1.1]); // 仕切りを出す
            ops.push(vec![1, 0, 0, i1]); // 絵の具を1グラム出す
            ops.push(vec![2, 0, 0]); // 画伯に渡す
            ops.push(vec![4, pos0.0, pos0.1, pos1.0, pos1.1]); // 仕切りを戻す
        }
    }

    if ops.len() > t {
        ops.pop(); // 最後の仕切りを戻すことは省略できる
    }

    (total_score, ops)
}

fn main() {
    input! {
        n: usize,
        k: usize,
        h: usize,
        t: usize,
        d: usize,
        cmy_own: [(f64, f64, f64); k],
        cmy_target: [(f64, f64, f64); h],
    }

    let v = build_pallet_v(n);
    let h = build_pallet_h(n);

    let mut candidates = vec![];
    for &(c_t, m_t, y_t) in &cmy_target {
        let cand = build_candidates(n, d, &cmy_own, (c_t, m_t, y_t));
        candidates.push(cand);
    }
    let (score0, results0) = build_results(n, t, d, &candidates);
    let (score1, results1) = build_another_results(n, t, d, &cmy_own, &cmy_target);
    eprintln!("socre0: {score0}");
    eprintln!("socre1: {score1}");

    for v in v {
        println!("{}", v.iter().join(" "));
    }
    for h in h {
        println!("{}", h.iter().join(" "));
    }
    let results = if score0 <= score1 { results0 } else { results1 };
    // let results = results0;
    for result in results {
        println!("{}", result.iter().join(" "));
    }
}
