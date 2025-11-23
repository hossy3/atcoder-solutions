use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        m: i64,
        s: Chars,
    }
    let mut i = 0i64; // Tシャツ合計数
    let mut i2 = 0i64; // ロゴ入りTシャツ合計数
    let mut max = 0i64;
    let mut max2 = 0i64;
    for &c in &s {
        match c {
            '0' => {
                i = 0;
                i2 = 0;
            }
            '1' => {
                i += 1;
                max = max.max(i);
            }
            '2' => {
                i += 1;
                max = max.max(i);
                i2 += 1;
                max2 = max2.max(i2);
            }
            _ => unreachable!(),
        }
    }
    let result = (max - m).max(max2);
    println!("{result}");
}
