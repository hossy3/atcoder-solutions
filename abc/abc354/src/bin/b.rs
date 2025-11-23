use proconio::input;

fn main() {
    input! {
        n: usize,
        mut sc: [(String, usize); n],
    }
    sc.sort();
    let sum: usize = sc.iter().map(|(_, c)| c).sum();
    let result = sc[sum % n].0.as_str();
    println!("{result}");
}
