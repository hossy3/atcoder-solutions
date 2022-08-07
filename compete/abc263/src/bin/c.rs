use proconio::input;

fn f(i: usize, val: usize, n: usize, m: usize, v: &mut [usize]) {
    v[i] = val + 1;
    if i == n - 1 {
        for x in v {
            print!("{} ", x);
        }
        println!();
        return;
    }
    for j in (val + 1)..m {
        f(i + 1, j, n, m, v);
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut v = vec![0; n];
    for i in 0..m {
        f(0, i, n, m, &mut v);
    }
}
