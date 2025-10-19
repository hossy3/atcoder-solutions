use proconio::input;

fn f(x: &String) -> usize {
    match x.as_str() {
        "Ocelot" => 1,
        "Serval" => 2,
        "Lynx" => 3,
        _ => unreachable!(),
    }
}

fn main() {
    input! {
        x: String,
        y: String,
    }
    let yes = f(&x) >= f(&y);
    println!("{}", if yes { "Yes" } else { "No" });
}
