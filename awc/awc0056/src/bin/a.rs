use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [isize; n],
        bs: [(Usize1, isize); m],
    }

    let result = (0..m).map(|i| a[bs[i].0] + bs[i].1).sum::<isize>();
    println!("{result}");
}
