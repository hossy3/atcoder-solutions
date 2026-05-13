use proconio::input;

fn expected_value_dp(
    n: usize,
    n1: usize,
    n2: usize,
    n3: usize,
    cache: &mut Vec<Vec<Vec<Option<f64>>>>,
) -> f64 {
    if n1 == 0 && n2 == 0 && n3 == 0 {
        return 0.0;
    }
    if let Some(cached) = cache[n1][n2][n3] {
        return cached;
    }

    let mut expected = 1.0; // まず 0枚の皿を選ばない場合を考える。1手増える
    if n1 > 0 {
        expected += expected_value_dp(n, n1 - 1, n2, n3, cache) * n1 as f64 / n as f64;
    }
    if n2 > 0 {
        expected += expected_value_dp(n, n1 + 1, n2 - 1, n3, cache) * n2 as f64 / n as f64;
    }
    if n3 > 0 {
        expected += expected_value_dp(n, n1, n2 + 1, n3 - 1, cache) * n3 as f64 / n as f64;
    }

    let k = n as f64 / (n1 + n2 + n3) as f64; // 0枚の皿を選んだ場合は状態が変わらないので、係数をかける
    expected *= k;
    cache[n1][n2][n3] = Some(expected);

    expected
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let n1 = a.iter().filter(|&&x| x == 1).count();
    let n2 = a.iter().filter(|&&x| x == 2).count();
    let n3 = a.iter().filter(|&&x| x == 3).count();
    let mut cache = vec![vec![vec![None; n3 + 1]; n2 + n3 + 1]; n + 1];
    let result = expected_value_dp(n, n1, n2, n3, &mut cache);
    println!("{result}");
}
