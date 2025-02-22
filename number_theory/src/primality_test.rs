use std::io::Write;

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut stdin = stdin.split_whitespace();
    let mut stdout = std::io::BufWriter::new(std::io::stdout().lock());

    let q: usize = stdin.next().unwrap().parse().unwrap();

    for _ in 0..q {
        let n: u64 = stdin.next().unwrap().parse().unwrap();
        let ans = is_prime(n);
        writeln!(stdout, "{}", if ans { "Yes" } else { "No" }).ok();
    }
}

pub fn is_prime(n: u64) -> bool {
    if n == 0 || n == 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    let r = (n - 1).trailing_zeros();
    let d = (n - 1) >> r;

    let miller_rabin = |a: u64| -> bool {
        let n = n as u128;
        let mut base = a as u128;
        let mut pow = 1;
        let mut d = d;
        while d > 0 {
            if d & 1 == 1 {
                pow *= base;
                pow %= n;
            }
            base *= base;
            base %= n;
            d >>= 1;
        }
        if pow == 1 || pow == n - 1 {
            return true;
        }
        for _ in 1..r {
            pow *= pow;
            pow %= n;
            if pow == n - 1 {
                return true;
            }
        }
        false
    };

    let a = if n < 4_759_123_141 {
        vec![2, 7, 61]
    } else {
        vec![2, 325, 9_375, 28_178, 450_775, 9_780_504, 1_795_265_022]
    };

    a.iter().filter(|&&a| a < n).all(|&a| miller_rabin(a))
}
