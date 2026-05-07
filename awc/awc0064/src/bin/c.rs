use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        rt: [(usize, usize); n],
    }

    let mut state = vec![0usize; m + 1]; // state[かけた時間] = 価値
    for &(r, t) in &rt {
        for i in (0..=(m - t)).rev() {
            state[i + t] = state[i + t].max(state[i] + r);
        }
    }

    let result = state[m];
    println!("{result}");
}
