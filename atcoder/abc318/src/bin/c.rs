use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        p: usize,
        mut f: [usize; n],
    }

    f.sort();
    let mut result: usize = f.iter().sum();
    while f.len() > 0 {
        let i = if f.len() < d { 0 } else { f.len() - d };
        let p0: usize = f[i..].iter().sum();
        if p0 > p {
            result -= p0 - p;
            f.truncate(i);
        } else {
            break;
        }
    }
    println!("{result}");
}
