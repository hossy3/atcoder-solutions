use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [usize; n],
    }
    let mut v = vec![false; 1001];
    for a in 1..=333 {
        for b in 1..=333 {
            let area = 4 * a * b + 3 * a + 3 * b;
            if area <= 1000 {
                v[area] = true;
            }
        }
    }
    let result = s.iter().filter(|&&x| !v[x]).count();
    println!("{}", result);
}
