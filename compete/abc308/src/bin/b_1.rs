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
        if let Some(j) = (0..m).find(|&j| d[j] == c[i]) {
            result += p[j + 1];
        } else {
            result += p[0];
        }
    }
    println!("{}", result);
}
