use ac_library::Dsu;
use proconio::{input, marker::Usize1};

fn f(n: usize, ab: &[(usize, usize)]) -> bool {
    let m = ab.len();
    if n != m {
        return false; // 頂点数と変数が異なる場合はサイクルではない
    }

    let mut count = 0usize;
    let mut num_edges = vec![0usize; n]; // 各頂点の接続する辺の数
    let mut dsu = Dsu::new(n);
    for &(a, b) in ab {
        num_edges[a] += 1;
        num_edges[b] += 1;
        if dsu.same(a, b) {
            count += 1;
        } else {
            dsu.merge(a, b);
        }
    }

    num_edges.iter().all(|&x| x == 2) && count == 1
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }
    let yes = f(n, &ab);
    println!("{}", if yes { "Yes" } else { "No" });
}
