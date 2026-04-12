use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

type Mint = ac_library::ModInt998244353;

fn combination(n: usize, r: usize, fact: &[Mint], fact_inv: &[Mint]) -> Mint {
    fact[n] * fact_inv[r] * fact_inv[n - r]
}

fn build_fact(n: usize) -> Vec<Mint> {
    let mut fact = vec![Mint::new(1); n + 1];
    for i in 2..=n {
        fact[i] = fact[i - 1] * i;
    }
    fact
}

fn build_fact_inv(fact: &[Mint]) -> Vec<Mint> {
    let n = fact.len() - 1;
    let mut fact_inv = vec![Mint::new(1); n + 1];
    for i in 0..=n {
        fact_inv[i] = fact[i].inv();
    }
    fact_inv
}

fn has_intersect((x0, x1): (usize, usize), (x2, x3): (usize, usize)) -> bool {
    x0.max(x2) < x1.min(x3)
}

fn main() {
    input! {
        n: usize,
        lr: [(usize, usize); n],
    }

    let fact = build_fact(n);
    let fact_inv = build_fact_inv(&fact);

    let mut result = Mint::new(0);
    let mut heap_a_in = BinaryHeap::new();
    let mut heap_b_in = BinaryHeap::new();
    let mut heap_w_in = BinaryHeap::new();
    for &(l, r) in &lr {
        let (l0, r0) = (n - r, n - l);
        heap_a_in.push(Reverse((l, r + 1)));
        heap_b_in.push(Reverse((l0, r0 + 1)));
        if has_intersect((l, r + 1), (l0, r0 + 1)) {
            heap_w_in.push(Reverse((l.min(l0), r.max(r0) + 1)));
        } else {
            heap_w_in.push(Reverse((l, r + 1)));
            heap_w_in.push(Reverse((l0, r0 + 1)));
        }
    }
    // eprintln!("{:?} {:?} {:?}", &heap_a_in, &heap_b_in, &heap_w_in);

    let mut heap_a_out = BinaryHeap::new();
    let mut heap_b_out = BinaryHeap::new();
    let mut heap_w_out = BinaryHeap::new();
    let mut count_a = 0;
    let mut count_b = 0;
    let mut count_w = 0;
    for i in 1..n {
        while let Some(&Reverse((l, r))) = heap_a_in.peek() {
            if i == l {
                count_a += 1;
                heap_a_out.push(Reverse(r));
                heap_a_in.pop();
            } else {
                break;
            }
        }
        while let Some(&Reverse((l, r))) = heap_b_in.peek() {
            if i == l {
                count_b += 1;
                heap_b_out.push(Reverse(r));
                heap_b_in.pop();
            } else {
                break;
            }
        }
        while let Some(&Reverse((l, r))) = heap_w_in.peek() {
            if i == l {
                count_w += 1;
                heap_w_out.push(Reverse(r));
                heap_w_in.pop();
            } else {
                break;
            }
        }
        while let Some(&Reverse(r)) = heap_a_out.peek() {
            if i == r {
                count_a -= 1;
                heap_a_out.pop();
            } else {
                break;
            }
        }
        while let Some(&Reverse(r)) = heap_b_out.peek() {
            if i == r {
                count_b -= 1;
                heap_b_out.pop();
            } else {
                break;
            }
        }
        while let Some(&Reverse(r)) = heap_w_out.peek() {
            if i == r {
                count_w -= 1;
                heap_w_out.pop();
            } else {
                break;
            }
        }

        if count_w == n && count_a >= i && count_b >= n - i {
            result += combination(count_a + count_b - n, count_a - i, &fact, &fact_inv);
            // eprintln!("{count_a} {count_b} {n} {i}");
        }
    }

    println!("{result}");
}
