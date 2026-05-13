use proconio::{input, marker::Chars};

type Mint = ac_library::ModInt1000000007;

fn grid_dp(a: &[Vec<char>]) -> Mint {
    let w = a[0].len();

    let mut state = vec![Mint::new(0); w];
    state[0] = Mint::new(1);
    for a in a {
        let mut next = vec![Mint::new(0); w];
        for (j, &a) in a.iter().enumerate() {
            if a == '#' {
                continue;
            }
            next[j] = state[j];
            if j > 0 {
                let x = next[j - 1];
                next[j] += x;
            }
        }
        state = next;
    }

    state[w - 1]
}

fn main() {
    input! {
        h: usize,
        _w: usize,
        a: [Chars; h],
    }

    let result = grid_dp(&a);
    println!("{result}");
}
