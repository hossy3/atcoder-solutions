use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [isize; n],
        xy: [(isize, isize); q],
    }

    let mut a = a.clone();
    a.sort();
    let a: Vec<_> = a.iter().enumerate().map(|(i, &x)| (x, i)).collect();

    for &(x, y) in &xy {
        let result = match a.binary_search_by_key(&x, |&(x, _)| x) {
            Ok(i) => {
                let j = i + a[i..(n - 1)]
                    .partition_point(|&(_, j)| y + a[i].0 + ((j - i) as isize) >= a[j + 1].0);
                y + a[i].0 + (j - i) as isize
            }
            Err(i) => {
                if i == 0 {
                    if x + y - 1 < a[0].0 {
                        x + y - 1
                    } else {
                        let y = y + (x - a[0].0);
                        let j = i + a[i..(n - 1)].partition_point(|&(_, j)| {
                            y + a[i].0 + ((j - i) as isize) >= a[j + 1].0
                        });
                        y + a[i].0 + (j - i) as isize
                    }
                } else {
                    let i = i - 1;
                    let y = y + (x - a[i].0) - 1;
                    let j = i + a[i..(n - 1)]
                        .partition_point(|&(_, j)| y + a[i].0 + ((j - i) as isize) >= a[j + 1].0);
                    y + a[i].0 + (j - i) as isize
                }
            }
        };

        println!("{result}");
    }
}
