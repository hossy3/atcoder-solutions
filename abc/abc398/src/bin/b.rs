use proconio::input;

fn main() {
    input! {
        a: [usize; 7],
    }
    let mut v = vec![0usize; 13];
    for x in a {
        v[x - 1] += 1;
    }
    let yes =
        v.iter().filter(|&&x| x >= 2).count() >= 2 && v.iter().filter(|&&x| x >= 3).count() >= 1;
    println!("{}", if yes { "Yes" } else { "No" });
}
