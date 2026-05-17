use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [char; n],
    }
    let r = s.iter().filter(|&&c| c == 'R').count();
    let w = s.iter().filter(|&&c| c == 'W').count();
    let q = s.iter().filter(|&&c| c == '?').count();
    let result = if r.abs_diff(w) <= q {
        n % 2
    } else {
        if r > w { r - (w + q) } else { w - (r + q) }
    };
    println!("{result}");
}
