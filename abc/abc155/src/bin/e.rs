use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
    }
    let mut a = [1usize, 0usize]; // [1枚上の桁を余らせたときの交換数, 余らせないときの交換数]
    for c in n {
        let i = c as usize - '0' as usize;
        a = [
            (a[0] + 10 - i - 1).min(a[1] + i + 1),
            (a[0] + 10 - i).min(a[1] + i),
        ];
    }
    let result = a[1];
    println!("{result}");
}
