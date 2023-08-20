use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [i64; n]
    }

    let mut imos = vec![0; n];
    for (i, &x) in p.iter().enumerate() {
        let j = (n + x as usize - i) % n;
        let k0 = (j + n / 2) % n;
        let k1 = (j + n - n / 2) % n;
        if j == 0 {
            imos[j] += 1;
            imos[k0] -= 1;
            imos[k1] -= 1;
        } else if j == n / 2 {
            imos[k1] -= 1;
            imos[j] += 2;
        } else {
            if j < n / 2 {
                imos[0] -= 1;
            } else {
                imos[0] += 1;
            }
            if k1 == 0 {
                imos[0] += 1;
            }
            imos[j] += 2;
            imos[k0] -= 1;
            imos[k1] -= 1;
        }
    }
    let mut scores = vec![0; n];
    for (i, &x) in p.iter().enumerate() {
        let j = ((n + x as usize - i) % n).min((n - x as usize + i) % n);
        scores[0] += j as i64;
    }
    let mut sum = 0;
    for i in 1..n {
        sum += imos[i - 1];
        scores[i] = scores[i - 1] + sum;
    }

    let score = scores.iter().fold(1 << 60, |acc, x| acc.min(*x));
    println!("{}", score);
}
