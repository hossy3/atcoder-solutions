use proconio::input;

// 最大値を返す
fn f(cur: usize, k: usize, a: &[usize]) -> usize {
    if k == 0 || a.len() == 0 {
        return cur;
    }
    let mut x = f(cur ^ a[0], k - 1, &a[1..]);
    if k < a.len() {
        x = x.max(f(cur, k, &a[1..]));
    }
    x
}

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let result = f(0, k, &a);
    println!("{result}");
}
