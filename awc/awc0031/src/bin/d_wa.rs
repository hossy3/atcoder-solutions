use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        l: [usize; n],
        r: [usize; m],
    }

    let yes = l.iter().sum::<usize>() >= r.iter().sum::<usize>();
    println!("{}", if yes { "Yes" } else { "No" });
}
