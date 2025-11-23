use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        st: [(usize, usize); n - 1],
    }
    for (i, &(s, t)) in st.iter().enumerate() {
        a[i + 1] += (a[i] / s) * t;
    } 
    let result = a[n - 1];
    println!("{result}");
}
