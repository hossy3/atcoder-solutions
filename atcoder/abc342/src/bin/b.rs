use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        p: [Usize1; n],
        q: usize,
        ab: [(Usize1, Usize1); q],
    }

    let mut v = vec![0; n];
    for (i, &x) in p.iter().enumerate() {
        v[x] = i; // 人 x は i 番目
    }
    for (a, b) in ab {
        let result = if v[a] < v[b] { a + 1 } else { b + 1 };
        println!("{result}");
    }
}
