use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let v: Vec<_> = (0..26)
        .map(|i| {
            s.iter()
                .position(|&c| c as u8 == 'A' as u8 + i as u8)
                .unwrap()
        })
        .collect();
    let result: i64 = v
        .windows(2)
        .map(|v| (v[0] as i64 - v[1] as i64).abs())
        .sum();
    println!("{result}");
}
