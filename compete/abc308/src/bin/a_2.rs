use proconio::input;

fn main() {
    input! {
        s: [usize; 8],
    }
    for i in 0..8 {
        let x = s[i];
        if !(100 <= x && x <= 675) || x % 25 != 0 {
            println!("No");
            return;
        }
    }
    for i in 0..7 {
        if s[i] > s[i + 1] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
