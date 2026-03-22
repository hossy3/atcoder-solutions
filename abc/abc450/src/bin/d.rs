use proconio::input;

fn f(k: usize, a: &[usize]) -> usize {
    let n = a.len();

    let mut v = vec![];
    for &x in a {
        v.push(x % k);
    }
    v.sort();

    if v[n - 1] == v[0] {
        return 0;
    }

    let mut result = usize::MAX;
    for i in 0..n {
        let j = (i + n - 1) % n;
        if v[i] == v[j] {
            continue;
        }
        let x = (v[j] + k - v[i]) % k;
        result = result.min(x);
    }
    result
}

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let result = f(k, &a);
    println!("{result}");
}
