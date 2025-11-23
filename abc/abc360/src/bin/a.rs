use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let ir = s.iter().position(|&c| c == 'R');
    let im = s.iter().position(|&c| c == 'M');
    let yes = ir < im;
    println!("{}", if yes { "Yes" } else { "No" });
}
