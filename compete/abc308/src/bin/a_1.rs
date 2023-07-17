use proconio::input;

fn main() {
    input! {
        s: [usize; 8],
    }
    for &x in &s {
        if !(100 <= x && x <= 675) || x % 25 != 0 {
            println!("No");
            return;
        }
    }
    for v in s.windows(2) {
        if v[0] > v[1] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
