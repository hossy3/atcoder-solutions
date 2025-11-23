use itertools::Itertools;
use proconio::input;

// 左端から移動し、カギをかけて右で止まって良いときの、カギをかける手数
fn f(a: &[u8]) -> usize {
    if a.iter().all(|&x| x == 1) {
        0
    } else {
        let r = a.iter().rposition(|&x| x == 0).unwrap_or(a.len());
        let count = a[0..r].iter().filter(|&&x| x == 1).count() + r + 1;
        count
    }
}

fn main() {
    input! {
        n: usize,
        r: usize,
        l: [u8; n],
    }
    let v0 = l[..r].iter().rev().cloned().collect_vec();
    let v1 = l[r..].to_vec();
    let result = f(&v0) + f(&v1);
    println!("{result}");
}
