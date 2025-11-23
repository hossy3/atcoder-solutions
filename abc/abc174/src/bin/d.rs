use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        c: Chars,
    }
    let r_num = c.iter().filter(|&&c| c == 'R').count();
    let result = c[..r_num].iter().filter(|&&c| c != 'R').count();
    println!("{result}");
}
