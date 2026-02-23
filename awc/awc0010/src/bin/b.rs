use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [usize; n],
    }
    let result = d[0]
        + d.windows(2)
            .map(|v| if v[0] < v[1] { v[1] / 2 } else { v[1] })
            .sum::<usize>();
    println!("{result}");
}
