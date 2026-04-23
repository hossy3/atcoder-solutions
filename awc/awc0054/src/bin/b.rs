use proconio::input;

fn main() {
    input! {
        n: usize,
        _w: usize,
        mut lw: [(usize, usize); n],
    }

    lw.sort_unstable();

    let mut v = vec![lw[0]];
    for &(l, w) in lw.iter().skip(1) {
        let Some(&(l0, w0)) = v.last() else {
            unreachable!()
        };
        if l0 + w0 >= l {
            v.pop();
            // eprintln!("pop: {l0}, {w0}");
            let w0 = w0.max(l + w - l0);
            v.push((l0, w0));
            // eprintln!("push: {l0}, {w0}");
        } else {
            v.push((l, w));
            // eprintln!("push: {l}, {w}");
        }
    }

    let result = v.iter().map(|(_, w)| w).sum::<usize>();
    println!("{result}");
}
