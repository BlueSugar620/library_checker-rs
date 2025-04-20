use std::io::Write;

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut stdin = stdin.split_whitespace();
    let mut stdout = std::io::BufWriter::new(std::io::stdout().lock());

    let n: usize = stdin.next().unwrap().parse().unwrap();
    let m: usize = stdin.next().unwrap().parse().unwrap();
    let s: usize = stdin.next().unwrap().parse().unwrap();
    let t: usize = stdin.next().unwrap().parse().unwrap();

    let e: Vec<(usize, usize, u64)> = (0..m)
        .map(|_| {
            (
                stdin.next().unwrap().parse().unwrap(),
                stdin.next().unwrap().parse().unwrap(),
                stdin.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let dijkstra = Dijkstra::new(n, &e, s);
    let ans = dijkstra.pass(t);
    if ans == None {
        writeln!(stdout, "{}", -1).ok();
    } else {
        let (dist, pass) = ans.unwrap();
        writeln!(stdout, "{} {}", dist, pass.len() - 1).ok();
        for v in pass.windows(2) {
            writeln!(stdout, "{} {}", v[0], v[1]).ok();
        }
    }
}

pub struct Dijkstra {
    dist: Vec<u64>,
    prev: Vec<usize>,
}

impl Dijkstra {
    pub fn new(n: usize, e: &[(usize, usize, u64)], source: usize) -> Self {
        let _e = e;
        let mut e = vec![vec![]; n];
        for &(u, v, d) in _e {
            e[u].push((v, d));
        }

        let mut dist = vec![!0; n];
        let mut prev = vec![n; n];
        let mut heap = std::collections::BinaryHeap::new();

        dist[source] = 0;
        prev[source] = source;
        heap.push((!0, source));

        while let Some((d, u)) = heap.pop() {
            let d = !d;
            if dist[u] < d {
                continue;
            }
            for &(v, dd) in &e[u] {
                let d = d + dd;
                if d < dist[v] {
                    dist[v] = d;
                    prev[v] = u;
                    heap.push((!d, v));
                }
            }
        }

        Self { dist, prev }
    }

    pub fn distance(&self, to: usize) -> Option<u64> {
        if self.dist[to] < !0 {
            Some(self.dist[to])
        } else {
            None
        }
    }

    pub fn pass(&self, to: usize) -> Option<(u64, Vec<usize>)> {
        if self.dist[to] == !0 {
            None
        } else {
            let mut now = to;
            let mut res = vec![to];
            while self.prev[now] != now {
                now = self.prev[now];
                res.push(now);
            }
            res.reverse();
            Some((self.dist[to], res))
        }
    }
}
