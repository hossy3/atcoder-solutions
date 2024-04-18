use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        qn: usize,
        xy: [(i64, i64); n],
        q: [Usize1; qn],
    }

    let add_min = xy.iter().map(|&(x, y)| x + y).min().unwrap();
    let add_max = xy.iter().map(|&(x, y)| x + y).max().unwrap();
    let sub_min = xy.iter().map(|&(x, y)| x - y).min().unwrap();
    let sub_max = xy.iter().map(|&(x, y)| x - y).max().unwrap();

    for i in q {
        let (x, y) = xy[i];
        let add = x + y;
        let sub = x - y;
        let a = [add_max - add, add - add_min, sub_max - sub, sub - sub_min];
        let result = a.iter().max().unwrap();
        println!("{result}");
    }
}
