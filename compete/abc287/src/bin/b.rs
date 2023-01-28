use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [usize; n],
        t: [usize; m],
    }
    let mut count = 0usize;
    'outer: for &s in &s {
        let s = s % 1000;
        for &t in &t {
            if s == t {
                count += 1;
                continue 'outer;
            }
        }
    }
    println!("{}", count);
}
