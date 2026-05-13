use proconio::input;

fn f(mut n: usize) -> usize {
    let mut v = vec![];
    while n > 0 {
        v.push(n % 10);
        n /= 10;
    }

    let mut state = (1usize, 0usize); // 上の桁から1借りるときの個数 (上の桁分も数える), 借りないときの個数
    while let Some(n) = v.pop() {
        let prev = state;
        state.0 = (prev.1 + n + 1).min(prev.0 + 9 - n);
        state.1 = (prev.1 + n).min(prev.0 + 10 - n);
        // eprintln!("{}: {:?}", n, state);
    }
    (state.0 + 1).min(state.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert_eq!(f(5), 5);
        assert_eq!(f(45), 9);
        assert_eq!(f(55), 9);
        assert_eq!(f(555), 13);
    }
}

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
        }
        let result = f(n);
        println!("{result}");
    }
}
