use std::io::Write;

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut stdin = stdin.split_whitespace();
    let mut stdout = std::io::BufWriter::new(std::io::stdout().lock());

    let n: usize = stdin.next().unwrap().parse().unwrap();
    let a: Vec<u64> = (0..n)
        .map(|_| stdin.next().unwrap().parse().unwrap())
        .collect();

    let (cnt, lis) = strictly_longest_increase(&a, std::u64::MAX);
    writeln!(stdout, "{}", cnt).ok();
    for lis in lis {
        write!(stdout, "{} ", lis).ok();
    }
}

pub fn strictly_longest_increase<T: Copy + PartialOrd>(a: &[T], inf: T) -> (usize, Vec<usize>) {
    let n = a.len();
    let mut dp = vec![inf; n + 1];
    let mut idx = vec![];

    for &a in a {
        let mut l = 0;
        let mut r = n;
        while r - l > 1 {
            let o = (l + r) / 2;
            if dp[o] < a {
                l = o;
            } else {
                r = o;
            }
        }
        dp[r] = a;
        idx.push(r);
    }

    let mut lis = vec![];
    let mut pos = dp.iter().rposition(|dp| *dp < inf).unwrap();
    for (i, &idx) in idx.iter().enumerate().rev() {
        if idx == pos {
            lis.push(i);
            if pos == 0 {
                break;
            } else {
                pos -= 1;
            }
        }
    }
    lis.reverse();
    (lis.len(), lis)
}
