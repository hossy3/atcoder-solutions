use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }

    let mut result = 1usize;
    let mut v = vec![0usize; n]; // 出現数

    for i in 0..n {
        // 前と同じ数字なら入れ替えても意味がないのでスキップする
        if i > 0 && a[i] == a[i - 1] {
            v[a[i]] += 1;
            continue;
        }

        // eprintln!("{result}");
        result += i;
        result -= v[a[i]];
        v[a[i]] += 1;
    }

    println!("{result}");
}
