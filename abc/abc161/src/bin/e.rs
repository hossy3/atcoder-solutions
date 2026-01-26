use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        k: usize,
        c: usize,
        s: Chars,
    }

    // 一番早く働く場合
    let mut i = 0;
    let mut v0 = vec![];
    while v0.len() < k {
        if s[i] == 'o' {
            v0.push(i + 1);
            i += c;
        }
        i += 1;
    }

    // 一番遅く働く場合
    i = n - 1;
    let mut v1 = vec![];
    while v1.len() < k {
        if s[i] == 'o' {
            v1.push(i + 1);
            i = i.wrapping_sub(c);
        }
        i = i.wrapping_sub(1);
    }
    v1.reverse();

    // 同じ場合は必ず働く日
    for (x0, x1) in v0.iter().zip(v1.iter()) {
        if x0 == x1 {
            println!("{x0}");
        }
    }
}
