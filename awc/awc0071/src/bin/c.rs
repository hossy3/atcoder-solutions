use proconio::input;

fn main() {
    input! {
        n: usize,
        w: [usize; n],
    }

    let sum = w.iter().sum::<usize>();

    'outer: for k in (1..=n).rev() {
        if sum % k > 0 {
            continue;
        }
        let w0 = sum / k; // 目標の重さ

        let mut cur = 0;
        for &w in &w {
            cur += w;
            if cur > w0 {
                continue 'outer;
            }
            if cur == w0 {
                cur = 0;
            }
        }
        println!("{k}");
        return;
    }
}
