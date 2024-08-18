use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }
    let yes = if b < c {
        (a < b) || (c < a) 
    } else {
        (b < a) && (a < c)
    };
    println!("{}", if yes { "Yes" } else { "No" });
}
