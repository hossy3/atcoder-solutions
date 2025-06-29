use proconio::{input, marker::Chars};

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            s: Chars,
        }

        // 3状態のDPにする
        // 0: 1現れていない
        // 1: 1の途中
        // 2: 1終わった
        let mut sc = (0, 0, 0);
        for i in 0..n {
            sc = match s[i] {
                '0' => (sc.0, sc.1 + 1, sc.1.min(sc.2)),
                '1' => (sc.0 + 1, sc.0.min(sc.1), sc.2 + 1),
                _ => unreachable!(),
            }
        }
        let result = sc.0.min(sc.1).min(sc.2);
        println!("{result}");
    }
}
