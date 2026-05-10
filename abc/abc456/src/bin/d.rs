use proconio::{input, marker::Chars};

type Mint = ac_library::ModInt998244353;

fn main() {
    input! {
        s: Chars,
    }

    let mut state = (Mint::new(0), Mint::new(0), Mint::new(0));
    for c in s {
        state = match c {
            'a' => (state.0 + state.1 + state.2 + 1, state.1, state.2),
            'b' => (state.0, state.0 + state.1 + state.2 + 1, state.2),
            'c' => (state.0, state.1, state.0 + state.1 + state.2 + 1),
            _ => unreachable!(),
        }
    }

    let result = state.0 + state.1 + state.2;
    println!("{result}");
}
