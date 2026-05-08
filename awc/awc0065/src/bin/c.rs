use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut state = (0, 0); // 直前植えてない、直前植えた
    for &a in &a {
        state = (state.0.max(state.1), state.0 + a);
    }
    let result = state.0.max(state.1);
    println!("{result}");
}
