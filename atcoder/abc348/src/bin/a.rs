use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut v = vec!['o'; n];
    for i in 0..n {
        if i % 3 == 2 {
            v[i] = 'x';
        }
    }
    let result = v.iter().join("");
    println!("{result}");
}
