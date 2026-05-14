use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[usize; m]; n],
    }

    let mut result = 1usize;
    let mut max = 0usize;
    for (i, a) in a.iter().enumerate() {
        let x = a.windows(2).map(|v| v[0].abs_diff(v[1])).sum::<usize>();
        if x > max {
            result = i + 1;
            max = x;
        }
    }
    println!("{result}");
}
