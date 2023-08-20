use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        mut a: [usize],
        x: [usize],
    }
    a.sort();
    for x in &x {
        let result = a.lower_bound(x);
        println!("{}", result);
    }
}
