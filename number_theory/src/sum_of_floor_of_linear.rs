use std::io::Write;

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut stdin = stdin.split_whitespace();
    let mut stdout = std::io::BufWriter::new(std::io::stdout().lock());

    let t: u32 = stdin.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let n: i64 = stdin.next().unwrap().parse().unwrap();
        let m: i64 = stdin.next().unwrap().parse().unwrap();
        let a: i64 = stdin.next().unwrap().parse().unwrap();
        let b: i64 = stdin.next().unwrap().parse().unwrap();

        let ans = floor_sum(n, m, a, b);
        writeln!(stdout, "{}", ans).ok();
    }
}

pub fn floor_sum(mut n: i64, mut m: i64, mut a: i64, mut b: i64) -> i64 {
    let mut res = 0;
    while m > 0 {
        if a >= m {
            res += a / m * n * (n - 1) / 2;
            a %= m;
        }
        if b >= m {
            res += b / m * n;
            b %= m;
        }
        let k = a * n + b;
        if k < m {
            break;
        }
        n = k / m;
        b = k % m;
        std::mem::swap(&mut a, &mut m);
    }
    res
}
