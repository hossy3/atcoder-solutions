use im_rc::HashMap;
use proconio::input;

fn f(i: usize, j: usize, x: usize, ms: &mut [HashMap<usize, usize>], a: &Vec<Vec<usize>>) {
    let n = a.len();
    let x = x ^ a[i][j];
    if i + j + 1 == n {
        ms[i].insert(x, ms[i].get(&x).unwrap_or(&0) + 1);
    } else {
        f(i + 1, j, x, ms, a);
        f(i, j + 1, x, ms, a);
    }
}

fn g(i: usize, j: usize, x: usize, ms: &mut [HashMap<usize, usize>], a: &Vec<Vec<usize>>) {
    let n = a.len();
    if i + j + 1 == n {
        ms[i].insert(x, ms[i].get(&x).unwrap_or(&0) + 1);
    } else {
        let x = x ^ a[i][j];
        g(i - 1, j, x, ms, a);
        g(i, j - 1, x, ms, a);
    }
}

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n]
    }
    let mut ms0 = vec![HashMap::new(); n];
    f(0, 0, 0, &mut ms0, &a);
    let mut ms1 = vec![HashMap::new(); n];
    g(n - 1, n - 1, 0, &mut ms1, &a);

    let mut count = 0;
    for i in 0..n {
        for &(k, x) in &ms0[i] {
            if ms1[i].contains_key(&k) {
                count += ms1[i][&k] * x;
            }
        }
    }
    println!("{}", count);
}
