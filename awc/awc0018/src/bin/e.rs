use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        b: usize,
        cs: [(usize, usize); n],
    }

    let mut state = vec![vec![usize::MAX; b + 1]; k + 1]; // [登った山の数][使った予算]=最大標高
    state[0][0] = 0;

    for (c, s) in cs {
        for i in (0..k).rev() {
            for j in 0..=(b - c) {
                if state[i][j] < s && state[i + 1][j + c] > s {
                    state[i + 1][j + c] = s;
                }
            }
        }
    }

    let result = (0..=k)
        .rfind(|&i| state[i].iter().any(|&x| x < usize::MAX))
        .unwrap_or(0);
    println!("{result}");
}
