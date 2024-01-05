use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [[usize; m]; n],
    }

    for i in 0..n {
        a[i].sort();
    }
    let mut result = 0usize;
    for i in 0..(n - 1) {
        for j in (i + 1)..n {
            let mut j0 = 0;
            for i0 in 0..m {
                while j0 < m && a[j][j0] < a[i][i0] {
                    j0 += 1;
                }
                result += i0 + j0 + 1;
            }
        }
    }
    println!("{}", result);
}
