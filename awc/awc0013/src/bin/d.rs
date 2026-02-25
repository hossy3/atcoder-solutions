use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[isize; m]; n],
    }

    // 各次数について求めて合計を出す
    let mut result = 0isize;
    for i in 0..m {
        let mut v = vec![];
        for j in 0..n {
            v.push(a[j][i]);
        }
        v.sort_unstable();

        let mut cum = vec![0isize; n + 1];
        for j in 0..n {
            cum[j + 1] = cum[j] + v[j];
        }

        let mut sum = 0isize;
        for j in 0..n {
            sum += v[j] * j as isize - cum[j];
        }
        result += sum;
    }
    println!("{result}");
}
