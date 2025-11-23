use proconio::input;

// 最大値を返す
fn f(cur: usize, k: usize, a: &[usize]) -> usize {
    if k == 0 || a.len() == 0 {
        return cur;
    }
    let mut x = f(cur ^ a[0], k - 1, &a[1..]);
    if k < a.len() && (cur | (2usize.pow(a[1].ilog2() + 1) - 1) > x) {
        x = x.max(f(cur, k, &a[1..]));
    }
    x
}

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n],
    }
    a.sort();
    a.reverse();
    let result = f(0, k, &a);
    println!("{result}");
}
