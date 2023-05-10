use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
        x: [usize; q],
    }
    a.sort();
    for x in x {
        let result = n - a.lower_bound(&x);
        println!("{}", result);
    }
}
