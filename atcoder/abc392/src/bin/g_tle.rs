use proconio::input;

fn main() {
    input! {
        n: usize,
        mut s: [usize; n],
    }
    s.sort();
    let mut result = 0usize;
    for i in 0..(n - 1) {
        for j in (i + 1)..n {
            let x = s[j] * 2 - s[i];
            if s.binary_search(&x).is_ok() {
                result += 1;
            } else if x > s[n - 1] {
                break;
            }
        }
    }
    println!("{result}");
}
