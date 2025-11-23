use proconio::input;

fn f(w: &[Vec<usize>], d: &[Vec<usize>], k0: usize, k1: usize) -> Vec<char> {
    let n = w.len();

    let mut paths = vec![(0usize, 0usize)];
    let mut targets = vec![vec![true; n]; n];
    targets[0][0] = false; // (0, 0) には置かれていない
    let mut lest = n * n - 1;

    while lest > 0 {
        let mut wd = vec![]; // [(上に積まれている重さの和, それぞれの耐久力)]

        // 残っている中で一番耐久力の高い荷物に進む
        {
            let (mut i0, mut j0) = (0, 0);
            for i in 0..n {
                for j in 0..n {
                    if targets[i][j] && (d[i0][j0] == 0 || d[i0][j0] < d[i][j]) {
                        (i0, j0) = (i, j);
                    }
                }
            }
            wd.push((0, d[i0][j0]));
            targets[i0][j0] = false;
            paths.push((i0, j0));
            lest -= 1;
        }

        // 近くの重い荷物を一緒に回収できるなら回収する
        loop {
            let Some(&(i0, j0)) = paths.last() else {
                unreachable!()
            };
            let (mut i1, mut j1) = (0, 0);
            let mut score1 = usize::MAX;
            for i in 0..n {
                'j: for j in 0..n {
                    if !targets[i][j] {
                        continue;
                    }
                    let w1 = w[i][j];
                    let mut score2 = 0;
                    for &(w2, d2) in &wd {
                        let cost2 = w2 * (i0.abs_diff(i) + j0.abs_diff(j)) + (w2 + w1) * (i + j);
                        if cost2 >= d2 {
                            continue 'j; // 持って帰れない
                        }
                        score2 += 30000 / d2.saturating_sub(w2)
                            * (((i0.abs_diff(i) + j0.abs_diff(j)) as f64)
                                .powf(k0 as f64 / k1 as f64))
                                as usize;
                    }
                    if score1 > score2 {
                        (i1, j1) = (i, j);
                        score1 = score2;
                    }
                }
            }
            if (i1, j1) == (0, 0) {
                break;
            }
            let k = wd.len();
            let (w1, d1) = (w[i1][j1], d[i1][j1]);
            for i in 0..k {
                wd[i].1 -= wd[i].0 * (i0.abs_diff(i1) + j0.abs_diff(j1));
                wd[i].0 += w1;
            }
            wd.push((0, d1));
            targets[i1][j1] = false;
            paths.push((i1, j1));
            lest -= 1;
        }

        // ゴールに戻る
        paths.push((0, 0));
    }

    // 出力を組み立てる
    let mut results = vec![];
    for v in paths.windows(2) {
        let (i0, j0) = v[0];
        let (i1, j1) = v[1];

        if i0 < i1 {
            for _ in i0..i1 {
                results.push('D');
            }
        } else if i0 > i1 {
            for _ in i1..i0 {
                results.push('U');
            }
        }

        if j0 < j1 {
            for _ in j0..j1 {
                results.push('R');
            }
        } else if j0 > j1 {
            for _ in j1..j0 {
                results.push('L');
            }
        }

        if (i1, j1) != (0, 0) {
            results.push('1');
        }
    }

    if results.len() > 2 * n * n * n {
        results.resize(2 * n * n * n, '1');
    }

    results
}

fn g(&&c: &&char) -> bool {
    c == 'U' || c == 'D' || c == 'L' || c == 'R'
}

fn main() {
    input! {
        n: usize,
        w: [[usize; n]; n],
        d: [[usize; n]; n],
    }

    let mut results = vec![];
    for k0 in 1..8 {
        for k1 in 1..8 {
            let results0 = f(&w, &d, k0, k1);
            if results.len() == 0
                || results.iter().filter(g).count() > results0.iter().filter(g).count()
            {
                eprintln!("{}", results0.len());
                results = results0;
            }
        }
    }

    for result in results {
        println!("{result}");
    }
}
