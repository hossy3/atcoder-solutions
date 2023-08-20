use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        n: usize,
        x: Usize1,
        a: Chars,
    }
    let mut l = x;
    while l > 0 && a[l - 1] == '.' {
        l -= 1;
    }
    let mut r = x + 1;
    while r < n && a[r] == '.' {
        r += 1;
    }
    for i in 0..l {
        print!("{}", a[i]);
    }
    for _ in l..r {
        print!("{}", '@');
    }
    for i in r..n {
        print!("{}", a[i]);
    }
    println!();
}
