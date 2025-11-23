use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        p: [f64; n],
    }

    // 1パックあけたときに何枚レアが出るか
    let mut v = vec![0.0_f64; n + 1];
    v[0] = 1.0;
    for (i, &p) in p.iter().enumerate() {
        let p0 = p / 100.0;
        for j in (0..=i).rev() {
            let p1 = v[j] * p0;
            v[j + 1] += p1;
            v[j] -= p1;
        }
    }

    // レアを残り i 枚以上得るための期待値を順に考える
    let mut results = vec![0.0_f64; x + 1];
    for i in 1..=x {
        let p0 = v[0]; // 1パックあけてレアを引けない
        let mut expected = 0.0_f64; // 残り回数の期待値
        for j in 1..=n {
            if j < i {
                expected += v[j] * (1.0 + results[i - j]);
            } else {
                expected += v[j];
            }
        }
        results[i] = (expected + p0) / (1.0 - p0);
    }

    println!("{}", results[x]);
}
