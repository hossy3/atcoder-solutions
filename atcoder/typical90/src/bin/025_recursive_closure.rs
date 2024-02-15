use proconio::input;

fn f0(mut n: usize) -> [usize; 10] {
    let mut a = [0usize; 10];
    while n > 0 {
        a[n % 10] += 1;
        n /= 10;
    }
    a
}

fn f1(v: &[usize]) -> [usize; 10] {
    let mut a = [0usize; 10];
    for &i in v {
        a[i] += 1;
    }
    a
}

fn combinations_with_replacement<F>(i: usize, start: usize, v: &mut [usize], f: &F) -> usize
where
    F: Fn(&[usize]) -> usize,
{
    if i == v.len() {
        f(v)
    } else {
        let mut result = 0;
        for j in start..=9 {
            v[i] = j;
            result += combinations_with_replacement(i + 1, j, v, f);
        }
        result
    }
}

fn f(n: usize, b: usize) -> usize {
    let ndigits = n.ilog10() as usize + 1;
    let mut result = 0usize;
    for i in 1..=ndigits {
        let mut v = vec![0; i];
        result += combinations_with_replacement(0, 0, &mut v, &|v| {
            let mul = v.iter().fold(1, |acc, &x| acc * x);
            let x = mul + b;
            if x <= n && f0(x) == f1(v) {
                1
            } else {
                0
            }
        });
    }
    result
}

fn main() {
    input! {
        n: usize,
        b: usize,
    }
    let result = f(n, b);
    println!("{result}");
}
