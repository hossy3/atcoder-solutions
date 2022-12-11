use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        d: usize,
        a: [i64; n],
    }
    let mut b = vec![vec![-1; d]; k + 1]; // mod d
    b[0][0] = 0;
    for i in 0..n {
        let mut b2 = Vec::new();
        for j in 0..=k {
            b2.push(b[j].clone());
        } 
        for j in 0..=(i.min(k - 1)) {
            for k in 0..d {
                if b2[j][k] >= 0 {
                    let y = (k + a[i] as usize) % d;
                    b[j + 1][y] = b[j + 1][y].max(b2[j][k] + a[i]);
                }
            }
        }
    }
    println!("{}", b[k][0]);
}
