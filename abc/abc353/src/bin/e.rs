use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut s: [Chars; n],
    }
    s.sort();
    let mut result = 0;
    for (i, str) in s.iter().enumerate() {
        let mut j = n - i - 1;
        let mut k = 0;
        while j > 0 && k < str.len() {
            j = s[(i + 1)..=(i + j)].partition_point(|str0| str0[k] == str[k]);
            result += j;
            k += 1;
        }
    }
    println!("{result}");
}
