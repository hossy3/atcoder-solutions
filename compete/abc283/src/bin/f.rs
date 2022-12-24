use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [i32; n],
    }
    for i in 0..n {
        let mut score = 10_000_000;
        for j in 1..n {
            if i >= j {
                let k = i - j;
                score = score.min((p[i] - p[k]).abs() + (j as i32));
            }
            if i + j < n {
                let k = i + j;
                score = score.min((p[i] - p[k]).abs() + (j as i32));
            }
            if j > score as usize {
                break;
            }
        }
        print!("{} ", score);
    }
    println!();
}
