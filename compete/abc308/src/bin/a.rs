use proconio::input;

fn main() {
    input! {
        s: [usize; 8],
    }
    let yes = s.windows(2).all(|v| v[0] <= v[1])
        && s.iter().all(|&x| 100 <= x && x <= 675 && (x % 25 == 0));
    println!("{}", if yes { "Yes" } else { "No" });
}
