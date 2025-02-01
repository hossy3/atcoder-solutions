use std::{cmp::Reverse, collections::BinaryHeap, mem::swap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n],
        mut b: [usize; n],
        mut c: [usize; n],
    }

    let mut heap = BinaryHeap::new();
    a.sort_by_key(|&x| Reverse(x));
    b.sort_by_key(|&x| Reverse(x));
    c.sort_by_key(|&x| Reverse(x));

    // 寄与の大きそうな法を先に回す
    if a[0] < b[0] {
        swap(&mut a, &mut b);
    }
    if b[0] < c[0] {
        swap(&mut b, &mut c);
    }
    if a[0] < b[0] {
        swap(&mut a, &mut b);
    }

    for &a in &a {
        if heap.len() >= k {
            let Some(&Reverse(x)) = heap.peek() else {
                unreachable!()
            };
            if a * b[0] + b[0] * c[0] + c[0] * a < x {
                break;
            }
        }
        for &b in &b {
            if heap.len() >= k {
                let Some(&Reverse(x)) = heap.peek() else {
                    unreachable!()
                };
                if a * b + b * c[0] + c[0] * a < x {
                    break;
                }
            }
            for &c in &c {
                let y = a * b + b * c + c * a;
                if heap.len() >= k {
                    let Some(&Reverse(x)) = heap.peek() else {
                        unreachable!()
                    };
                    if y < x {
                        break;
                    }
                    heap.pop();
                }
                heap.push(Reverse(y));
            }
        }
    }

    let Some(Reverse(result)) = heap.pop() else {
        unreachable!()
    };
    println!("{result}");
}
