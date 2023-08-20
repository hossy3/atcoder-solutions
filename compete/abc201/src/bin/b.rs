use proconio::input;

fn main() {
    input! {
        n: usize,
        mut st: [(String, usize); n],
    }
    st.sort_by_key(|x| x.1);
    let result = &st[st.len() - 2].0;
    println!("{}", result);
}
