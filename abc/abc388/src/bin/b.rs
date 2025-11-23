use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        tl: [(usize, usize); n],
    }
    for i in 1..=d {
        let Some(result) = tl.iter().map(|&(t, l)| t * (l + i)).max() else {
            unreachable!()
        };
        println!("{result}");
    }
}
