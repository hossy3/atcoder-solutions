use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
        z: usize,
    }
    let yes = if x <= y {
        false
    } else {
        let a = x - y;
        if a % (z - 1) > 0 {
            false
        } else {
            let b = a / (z - 1);
            y <= b
        }
    };
    println!("{}", if yes { "Yes" } else { "No" });
}
