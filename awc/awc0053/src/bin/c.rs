use proconio::input;

/// 座標圧縮 (Compress coordinates)
fn compress_coordinates(a: &[isize]) -> Vec<usize> {
    let mut sorted = a.to_vec();
    sorted.sort_unstable();
    sorted.dedup();
    let mut map = std::collections::HashMap::new();
    for (i, &x) in sorted.iter().enumerate() {
        map.insert(x, i);
    }
    a.iter().map(|&x| map[&x]).collect()
}

fn main() {
    input! {
        n: usize,
        xlrc: [(isize, isize, isize, isize); n],
    }

    let mut a = vec![];
    for &(x, l, r, _) in &xlrc {
        let l = x - l;
        let r = x + r + 1; // 半開区間にする
        a.push(l);
        a.push(r);
    }

    let a = compress_coordinates(&a);

    let m = *a.iter().max().unwrap();
    let mut imos = vec![0; m + 1];
    for i in 0..n {
        let l = a[2 * i];
        let r = a[2 * i + 1];
        let (_, _, _, c) = xlrc[i];
        imos[l] += c;
        imos[r] -= c;
    }
    for i in 1..=m {
        imos[i] += imos[i - 1];
    }

    let result = *imos.iter().max().unwrap();
    println!("{result}");
}
