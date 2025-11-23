use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }
    let mut v = [true; 24];
    if b < c {
        for i in b..c {
            v[i] = false;
        }
    } else {
        for i in 0..c {
            v[i] = false;
        }
        for i in b..24 {
            v[i] = false;
        }
    }
    let yes = v[a];
    println!("{}", if yes { "Yes" } else { "No" });
}
