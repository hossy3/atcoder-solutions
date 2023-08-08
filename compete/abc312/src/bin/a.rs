use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let yes = match s.as_str() {
        "ACE" | "BDF" | "CEG" | "DFA" | "EGB" | "FAC" | "GBD" => true,
        _ => false,
    };
    println!("{}", if yes { "Yes" } else { "No" });
}
