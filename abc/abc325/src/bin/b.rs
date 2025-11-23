use proconio::input;

fn main() {
    input! {
        n: usize,
        wx: [(usize, usize); n],
    }
    let mut a = [0usize; 24];
    for (w, x) in wx {
        for i in 9..18 {
            a[(x + i) % 24] += w;
        }
    }
    let result = a.iter().max().unwrap();
    println!("{result}");
}
