use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
    }

    let yes = a + b == 9.0 || a - b == 9.0 || a * b == 9.0 || a / b == 9.0;
    println!("{}", if yes { "Nine" } else { "Nein" });
}
