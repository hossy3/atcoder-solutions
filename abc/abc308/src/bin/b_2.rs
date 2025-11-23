use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        c: [String; n],
        d: [String; m],
        p: [usize; m + 1],
    }
    let mut result = 0;
    for i in 0..n {
        let j = (1..=m).find(|&j| d[j - 1] == c[i]).unwrap_or(0);
        result += p[j];
    }
    println!("{}", result);
}
