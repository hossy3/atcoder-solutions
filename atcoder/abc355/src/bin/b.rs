use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }
    let mut v = vec![];
    for x in a {
        v.push((x, true));
    }
    for x in b {
        v.push((x, false));
    }
    v.sort();
    let yes = v.windows(2).any(|x| x[0].1 && x[1].1);
    println!("{}", if yes { "Yes" } else { "No" });
}
