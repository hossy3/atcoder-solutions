use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            d: usize,
            a: [usize; n],
            b: [usize; n],
        }

        let mut cum = vec![0usize; n + 1]; // 仕入れた卵の合計
        for i in 0..n {
            cum[i + 1] = cum[i] + a[i];
        }

        let mut cur = 0usize;
        for i in 0..n {
            cur += a[i]; // 朝の行動
            cur -= b[i]; // 昼の行動
            if i + 1 >= d {
                cur = cur.min(cum[i + 1] - cum[i + 1 - d])
            }
        }
        println!("{cur}");
    }
}
