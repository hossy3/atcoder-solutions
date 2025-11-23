use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        s: Chars,
    }

    let mut a_cum = vec![0usize; n + 1];
    for i in 0..n {
        a_cum[i + 1] = a_cum[i] + if s[i] == 'a' { 1 } else { 0 };
    }

    let mut b_cum = vec![0usize; n + 1];
    for i in 0..n {
        b_cum[i + 1] = b_cum[i] + if s[i] == 'b' { 1 } else { 0 };
    }

    let mut result = 0;
    for i in 0..n {
        let a0 = a_cum[i];
        let a1 = a_cum[i..].partition_point(|&x| x - a0 < a);

        let b0 = b_cum[i];
        let b1 = b_cum[i..].partition_point(|&x| x - b0 < b);
        result += b1.saturating_sub(a1);
    }

    println!("{result}");
}
