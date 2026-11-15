use proconio::input;

fn main() {
    input! {
        _h: usize,
        _w: usize,
        n: usize,
        q: usize,
        rc: [(usize, usize); n],
    }

    let mut rows = vec![];
    let mut cols = vec![];
    for (r, c) in rc {
        rows.push(r);
        cols.push(c);
    }
    rows.sort_unstable();
    cols.sort_unstable();

    // eprintln!("rows: {:?}", &rows);
    // eprintln!("cols: {:?}", &cols);

    for _ in 0..q {
        input! {
            k: u8,
        }
        match k {
            1 => {
                input! {
                    u: usize,
                    d: usize,
                }
                let result = rows.partition_point(|&x| x <= d) - rows.partition_point(|&x| x < u);
                println!("{result}");
                // eprintln!(
                //     "1 {} {}: {} {}",
                //     u,
                //     d,
                //     rows.partition_point(|&x| x <= d),
                //     rows.partition_point(|&x| x < u)
                // );
            }
            2 => {
                input! {
                    l: usize,
                    r: usize,
                }
                let result = cols.partition_point(|&x| x <= r) - cols.partition_point(|&x| x < l);
                println!("{result}");
            }
            _ => unreachable!(),
        }
    }
}
