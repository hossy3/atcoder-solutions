use proconio::input;

fn f(c: char) -> usize {
    c as usize - 'A' as usize
}

fn main() {
    input! {
        p: char,
        q: char,
    }
    let a = [3, 1, 4, 1, 5, 9];
    let (l, r) = (f(p.min(q)), f(p.max(q)));
    let result: i64 = (l..r).map(|i| a[i]).sum();
    println!("{}", result);
}
