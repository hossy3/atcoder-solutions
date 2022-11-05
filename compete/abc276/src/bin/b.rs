use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }
    let mut g = vec![vec![]; n + 1];
    for &(a, b) in &ab {
        g[a].push(b);
        g[b].push(a);
    }
    for i in 1..=n {
        let mut v = g[i].clone();
        v.sort();
        print!("{}", v.len());
        for &x in &v {
            print!(" {}", x);
        }
        println!();
    }
}
