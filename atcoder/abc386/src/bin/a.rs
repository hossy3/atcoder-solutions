use proconio::input;

fn main() {
    input! {
        mut a: [usize; 4],
    }
    a.sort();
    let yes = (a[0] == a[1] && a[1] == a[2] && a[2] != a[3])
        || (a[0] == a[1] && a[1] != a[2] && a[2] == a[3])
        || (a[0] != a[1] && a[1] == a[2] && a[2] == a[3]);
    println!("{}", if yes { "Yes" } else { "No" });
}
