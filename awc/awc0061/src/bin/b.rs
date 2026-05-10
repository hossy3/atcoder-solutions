use proconio::input;

fn main() {
    input! {
        n: usize,
        mut s: [isize; n],
        c: [isize; n - 1],
    }

    for i in 0..(n - 1) {
        s[i + 1] += s[i] - c[i];
    }
    let result = *s.iter().max().unwrap();
    println!("{result}");
}
