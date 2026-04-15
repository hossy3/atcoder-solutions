use proconio::input;

fn main() {
    input! {
        n: usize,
        sk: [(String, usize); n],
    }
    for (s, k) in sk {
        let yes = (s.as_str() == "Yes" && k % 2 == 0) || (s.as_str() == "No" && k % 2 == 1);
        println!("{}", if yes { "Yes" } else { "No" });
    }
}
