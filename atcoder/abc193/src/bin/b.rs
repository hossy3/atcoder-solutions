use proconio::input;

fn main() {
    input! {
        n: usize,
        apx: [(usize, usize, usize); n],
    }
    let result: usize = apx
        .iter()
        .map(|&(a, p, x)| if x > a { p } else { std::usize::MAX })
        .min()
        .unwrap();
    if result == std::usize::MAX {
        println!("{}", -1);
    } else {
        println!("{}", result);
    }
}
