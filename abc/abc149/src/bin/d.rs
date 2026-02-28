use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        k: usize,
        r: usize,
        s: usize,
        p: usize,
        t: Chars,
    }

    // 余りごとに処理
    let mut result = 0;
    for i in 0..k {
        let mut a = [0; 3]; // 自分が出した手
        for j in 0..n {
            let j = j * k + i;
            if j >= n {
                break;
            }

            a = match t[j] {
                'r' => [a[1].max(a[2]), a[0].max(a[2]), (a[0] + p).max(a[1] + p)],
                's' => [(a[1] + r).max(a[2] + r), a[0].max(a[2]), a[0].max(a[1])],
                'p' => [a[1].max(a[2]), (a[0] + s).max(a[2] + s), a[0].max(a[1])],
                _ => unreachable!(),
            };
        }

        result += a[0].max(a[1]).max(a[2]);
    }

    println!("{result}");
}
