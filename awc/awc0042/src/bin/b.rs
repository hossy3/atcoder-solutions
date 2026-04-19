use proconio::input;

fn main() {
    input! {
        n: usize,
        mut s: usize,
        t: usize,
        mut d: [usize; n],
    }

    d.sort_unstable();
    for d in d {
        if s < d {
            break;
        }
        s += d;
    }

    let yes = s >= t;
    println!("{}", if yes { "Yes" } else { "No" });
}
