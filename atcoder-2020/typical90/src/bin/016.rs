use proconio::input;

const M: usize = 9999;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        c: usize,
    }
    let mut v = vec![a, b, c];
    v.sort_by(|a, b| b.cmp(a));

    let mut min_count = 1 << 60;
    let i_max = M.min(n / v[0]);
    for i in (0..=i_max).rev() {
        let j_max = M.min((n - i * v[0]) / v[1]);
        for j in (0..=j_max).rev() {
            if (n - i * v[0] - j * v[1]) % v[2] == 0 {
                let k = (n - i * v[0] - j * v[1]) / v[2];
                min_count = min_count.min(i + j + k);
                break;
            }
        }
    }
    println!("{}", min_count);
}
