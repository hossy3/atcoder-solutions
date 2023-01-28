use proconio::input;

fn main() {
    input! {
        a: [usize]
    }
    let mut v = [0; 100];
    for &a in &a {
        v[a % 100] += 1;
    }
    let mut result = 0;
    result += v[0] * (v[0] - 1) / 2;
    result += v[50] * (v[50] - 1) / 2;
    for i in 1..50 {
        result += v[i] * v[100 - i];
    }
    println!("{}", result);
}
