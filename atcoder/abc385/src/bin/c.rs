use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [usize; n],
    }

    let mut result = 1;
    for i in 0..n {
        for j in 1..(n - i) {
            let mut x = i + j;
            while x < n && h[x] == h[i] {
                x += j;
            }
            result = result.max((x - i) / j);
        }
    }
    println!("{result}");
}
