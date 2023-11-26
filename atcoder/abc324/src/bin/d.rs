use proconio::input;

// returns [#1, #2, ..., #9]
fn f(mut n: usize) -> [usize; 9] {
    let mut v = [0; 9];
    while n > 0 {
        let i = n % 10;
        n /= 10;
        if i > 0 {
            v[i - 1] += 1;
        }
    }
    v
}

fn main() {
    input! {
        n: u32,
        s: usize,
    }
    let n = 10_usize.pow(n);
    let v = f(s);
    let mut result = 0;
    let mut i = 0;
    while i * i < n {
        let w = f(i * i);
        if v == w {
            result += 1;
        }
        i += 1;
    }
    println!("{result}");
}
