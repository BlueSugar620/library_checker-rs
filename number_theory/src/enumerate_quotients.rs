use std::io::Write;

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut stdin = stdin.split_whitespace();
    let mut stdout = std::io::BufWriter::new(std::io::stdout().lock());

    let n: u64 = stdin.next().unwrap().parse().unwrap();
    let ans = quotients(n);
    writeln!(stdout, "{}", ans.len()).ok();
    for ans in ans {
        write!(stdout, "{} ", ans).ok();
    }
}

pub fn quotients(n: u64) -> Vec<u64> {
    let mut prefix = vec![];
    let mut suffix = vec![];
    for i in (1..).take_while(|i| i * i <= n) {
        prefix.push(i);
        if n / i != i {
            suffix.push(n / i);
        }
    }
    prefix.extend(suffix.iter().rev());
    prefix
}
