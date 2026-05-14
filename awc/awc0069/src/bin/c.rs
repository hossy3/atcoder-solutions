use proconio::{input, marker::Chars};

fn f(k: usize, s: &[char]) -> isize {
    let n = s.len();
    let mut v = vec![0usize; n + 1];
    for (i, &c) in s.iter().enumerate() {
        if c == '1' {
            v[i] += 1;
            v[i + 1] = 1;
        }
    }

    // eprintln!("{:?}", &v);
    let mut count = 0isize;
    for i in 0..=(n - k) {
        if v[i] % 2 == 0 {
            v[i] += 1;
            v[i + k] += 1;
            count += 1;
        }
        v[i + 1] += v[i];
    }
    // eprintln!("{:?}", &v);

    for i in (n - k + 1)..n {
        if v[i] % 2 == 0 {
            // eprintln!("{:?}", &v);
            return -1;
        }
        v[i + 1] += v[i];
    }
    eprintln!("{:?}", &v);
    count
}

fn main() {
    input! {
        _n: usize,
        k: usize,
        s: Chars,
    }
    let result = f(k, &s);
    println!("{result}");
}
