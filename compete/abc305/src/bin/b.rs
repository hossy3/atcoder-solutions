use proconio::input;

fn f(c: char) -> usize {
    c as usize - 'A' as usize
}

fn main() {
    input! {
        p: char,
        q: char,
    }
    let mut a = [0i64, 3, 1, 4, 1, 5, 9];
    for i in 0..(a.len() - 1) {
        a[i + 1] += a[i];
    }
    let result = (a[f(p)] - a[f(q)]).abs();
    println!("{}", result);
}
