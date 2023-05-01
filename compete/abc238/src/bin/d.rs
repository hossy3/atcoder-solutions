use proconio::input;

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            a: usize,
            s: usize,
        }
        let yes = (s >= a) && (((s - a) & a) == a);
        println!("{}", if yes { "Yes" } else { "No" });
    }
}
