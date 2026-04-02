use proconio::{input, marker::Usize1};

/// 半分全列挙
fn meet_in_the_middle(p: &[usize], uv: &[(usize, usize)]) -> usize {
    let n = p.len();
    assert!(n <= 40);

    // 隣接するものは同時に選べない
    let mut adj = vec![0usize; n];
    for &(u, v) in uv {
        adj[u] |= 1 << v;
        adj[v] |= 1 << u;
    }
    for i in 0..n {
        adj[i] |= 1 << i; // 自分自身を複数回足さないように、自分自身も隣接関係とする
    }

    let n0 = n / 2;
    let mut v0 = vec![0usize; 1 << n0]; // n1..n の範囲内だけでの最大値
    let mask0 = (1usize << n0) - 1;
    for i in 1..(1usize << n0) {
        let j = usize::BITS - i.leading_zeros() - 1; // 一番上の桁
        let i1 = i & (!adj[j as usize] & mask0); // i から隣接関係を消したもの。ここからなら移動できる
        v0[i] = v0[i & !(1 << j)].max(v0[i1] + p[j as usize]); // 一番上の桁の隣接を除いた最大値
    }
    // eprintln!("{:?}", &v0);

    let n1 = n - n0;
    let mut v1 = vec![0usize; 1 << n1]; // n1..n の範囲内だけでの最大値
    for i in 1..(1usize << n1) {
        let j = usize::BITS - i.leading_zeros() - 1; // 一番上の桁
        let i1 = i & (!adj[j as usize + n0] >> n0); // i から隣接関係を消したもの。ここからなら移動できる
        v1[i] = v1[i & !(1 << j)].max(v1[i1] + p[j as usize + n0]); // 一番上の桁の隣接を除いた最大値
    }
    // eprintln!("{:?}", &v1);

    let mut result = *v0.iter().max().unwrap();
    let mut adj1 = vec![0usize; 1 << n1]; // 0..n0 の範囲も含めた隣接関係
    for i in 1..(1usize << n1) {
        let j = usize::BITS - i.leading_zeros() - 1; // 一番上の桁
        adj1[i] = adj1[i & !(1 << j)] | adj[j as usize + n0];
        result = result.max(v1[i] + v0[!adj1[i] & mask0]);
    }

    result
}

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        p: [usize; n],
        uv: [(Usize1, Usize1); k],
    }

    let result = meet_in_the_middle(&p, &uv).min(m);
    println!("{result}");
}
