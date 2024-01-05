use proconio::input;

fn main() {
    input! {
        mut s: [String; 4],
    }
    s.sort();
    let expected = vec![
        String::from("2B"),
        String::from("3B"),
        String::from("H"),
        String::from("HR"),
    ];
    let yes = s == expected;
    println!("{}", if yes { "Yes" } else { "No" });
}
