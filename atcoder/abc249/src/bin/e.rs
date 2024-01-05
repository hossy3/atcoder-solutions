use proconio::input;

fn main() {
    input! {
        n: usize,
        p: usize,
    }
    let mut m = vec![vec![0_usize; n + 1001]; n + 6]; // m[rle][org]
    m[0][0] = 1;
    m[0][1] = p - 1;

    for i in 0..n {
        for j in 0..=n {
            let x = m[i][j];
            if x != 0 {
                let k = (if i == 0 { 26 } else { 25 } * x) % p;
                m[i + 2][j + 1] = (m[i + 2][j + 1] + k) % p;
                m[i + 2][j + 10] = (m[i + 2][j + 10] + p - k) % p;
                m[i + 3][j + 10] = (m[i + 3][j + 10] + k) % p;
                m[i + 3][j + 100] = (m[i + 3][j + 100] + p - k) % p;
                m[i + 4][j + 100] = (m[i + 4][j + 100] + k) % p;
                m[i + 4][j + 1000] = (m[i + 4][j + 1000] + p - k) % p;
                m[i + 5][j + 1000] = (m[i + 5][j + 1000] + k) % p;
            }
            m[i][j + 1] = (m[i][j + 1] + m[i][j]) % p;
        }
    }

    let mut sum = 0;
    for i in 0..n {
        sum = (sum + m[i][n]) % p;
    }
    println!("{}", sum);
}
