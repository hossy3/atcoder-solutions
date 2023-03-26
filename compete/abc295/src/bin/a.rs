use proconio::input;

fn main() {
    input! {
        n: usize,
        w: [String; n],
    }
    let yes = w.contains(&String::from("and"))
        || w.contains(&String::from("not"))
        || w.contains(&String::from("that"))
        || w.contains(&String::from("the"))
        || w.contains(&String::from("you"));
    println!("{}", if yes { "Yes" } else { "No" });
}
