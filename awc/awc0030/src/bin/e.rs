use proconio::input;

fn prime_division(mut n: usize) -> Vec<(usize, usize)> {
    let mut result = vec![];
    let mut k = 2;
    while k * k <= n {
        let mut count = 0;
        while n % k == 0 {
            count += 1;
            n /= k;
        }
        if count > 0 {
            result.push((k, count));
        }
        k += if k == 2 { 1 } else { 2 };
    }
    if n > 1 {
        result.push((n, 1));
    }
    result
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut v = vec![0usize; 1_000_000 + 1];
    for &x in &a {
        // すべての約数を列挙する
        let mut v0 = vec![1usize];
        for (p, k) in prime_division(x) {
            let mut v1 = vec![];
            for x0 in v0 {
                for k in 0..=k {
                    v1.push(x0 * p.pow(k as u32));
                }
            }
            v0 = v1;
        }

        for &x0 in &v0 {
            v[x0] += 1;
        }
    }

    let mut result = 0usize;
    for (i, &x) in v.iter().enumerate() {
        result = result.max(i * x);
    }
    println!("{result}");
}
