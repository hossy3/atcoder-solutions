use itertools::Itertools;
use proconio::input;

fn f(k: usize, x: usize, a: &[usize]) -> isize {
    let n = a.len();
    let a: Vec<_> = a.iter().sorted().collect();
    let mut x0 = 0;
    for i in (0..k).rev() {
        x0 += a[i];
        if x0 >= x {
            return (n - i) as isize;
        }
    }
    -1 // 小さなカップにお酒が入っていると満たせない
}

fn main() {
    input! {
        n: usize,
        k: usize,
        x: usize,
        a: [usize; n],
    }
    let result = f(k, x, &a);
    println!("{result}");
}
