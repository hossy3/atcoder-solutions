use proconio::input;

type Mint = ac_library::ModInt998244353;

fn main() {
    input! {
        n: usize,
        m: usize,
        d: [usize; n],
    }

    let mut state = vec![Mint::new(1)];
    for d in d {
        let prev = state;
        state = vec![Mint::new(0); prev.len() + 1];
        for (j, &count) in prev.iter().enumerate() {
            if d == 0 {
                state[j + 1] += count * (Mint::new(m) - j);
                state[j] += count * j; // 使用済みの値のどれでも入ってよい
            } else {
                if d <= j {
                    state[j] += count;
                } else if d == j + 1 {
                    state[j + 1] += count * (Mint::new(m) - j);
                }
            }
        }
        while let Some(&x) = state.last() {
            if x == Mint::new(0) {
                state.pop();
            } else {
                break;
            }
        }
        // eprintln!("{:?}", &state);
    }

    let result = state.iter().sum::<Mint>();
    println!("{result}");
}
