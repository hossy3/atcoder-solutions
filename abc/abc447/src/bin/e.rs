use ac_library::Dsu;
use proconio::{input, marker::Usize1};

type Mint = ac_library::ModInt998244353;

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    }

    let mut dsu = Dsu::new(n);
    let mut count = n; // グループ数
    let mut result = Mint::new(0);

    for (i, &(u, v)) in uv.iter().enumerate().rev() {
        if dsu.same(u, v) {
            continue; // 辺をつないでも良い
        }
        if count > 2 {
            dsu.merge(u, v); // 辺をつなぐ
            count -= 1;
        } else {
            result += Mint::new(2).pow((i + 1) as u64);
        }
    }

    println!("{result}");
}
