use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut v: Vec<(_, _)> = a.iter().enumerate().collect();
    v.sort_by_key(|(_, &x)| x);
    let result = v[v.len() - 2].0 + 1;
    println!("{result}");
}
