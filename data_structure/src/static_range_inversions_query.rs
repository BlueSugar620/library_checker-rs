use std::io::Write;

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut stdin = stdin.split_whitespace();
    let mut stdout = std::io::BufWriter::new(std::io::stdout().lock());

    let n: usize = stdin.next().unwrap().parse().unwrap();
    let q: usize = stdin.next().unwrap().parse().unwrap();
    let a: Vec<u64> = (0..n)
        .map(|_| stdin.next().unwrap().parse().unwrap())
        .collect();

    let inversions = StaticRangeInversions::new(&a, std::u64::MAX);

    let lr: Vec<(usize, usize)> = (0..q)
        .map(|_| {
            (
                stdin.next().unwrap().parse().unwrap(),
                stdin.next().unwrap().parse().unwrap(),
            )
        })
        .collect();
    for &(l, r) in &lr {
        writeln!(stdout, "{}", inversions.calc_inversions(l..r)).ok();
    }
}

use std::{
    cmp::Reverse,
    ops::{Bound, RangeBounds},
};

const CHUNK_SIZE: usize = 512;

pub struct StaticRangeInversions<T: Copy + PartialOrd> {
    n: usize,
    from_left: Vec<Vec<usize>>,
    from_right: Vec<Vec<usize>>,
    sorted_chunks: Vec<Vec<(usize, T)>>,
}

impl<T: Copy + PartialOrd> StaticRangeInversions<T> {
    pub fn new(a: &[T], inf: T) -> Self {
        let n = a.len();
        let mut a = a.to_vec();
        while a.len() % CHUNK_SIZE > 0 {
            a.push(inf);
        }

        let from_left = Self::create_block(&a);
        let mut from_right =
            Self::create_block(&a.iter().rev().map(|a| Reverse(*a)).collect::<Vec<_>>());
        from_right.reverse();

        let sorted_chunks = a
            .chunks(CHUNK_SIZE)
            .map(|a| {
                let mut a = a.iter().copied().enumerate().collect::<Vec<_>>();
                a.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
                a
            })
            .collect::<Vec<_>>();

        Self {
            n,
            from_left,
            from_right,
            sorted_chunks,
        }
    }

    pub fn calc_inversions<R: RangeBounds<usize>>(&self, range: R) -> usize {
        let (l, r) = unzip(range, self.n);
        let ll = l - l % CHUNK_SIZE;
        let rr = r + (CHUNK_SIZE - r % CHUNK_SIZE) % CHUNK_SIZE;
        self.from_left[ll / CHUNK_SIZE][r - ll]
            + self.from_right[rr / CHUNK_SIZE - 1][rr - l]
            + self.calc_ends(
                l % CHUNK_SIZE,
                r % CHUNK_SIZE,
                ll / CHUNK_SIZE,
                rr / CHUNK_SIZE,
            )
            - self.from_left[ll / CHUNK_SIZE][rr - ll]
    }

    fn create_block<S: Copy + PartialOrd>(a: &[S]) -> Vec<Vec<usize>> {
        let mut res: Vec<Vec<usize>> = vec![];
        let mut sorted_block = vec![];
        for (chunk_idx, chunk) in a.chunks(CHUNK_SIZE).enumerate().rev() {
            let sorted_chunk = {
                let mut sorted_chunk = (0..CHUNK_SIZE).collect::<Vec<_>>();
                sorted_chunk.sort_by(|i, j| chunk[*j].partial_cmp(&chunk[*i]).unwrap());
                sorted_chunk
            };
            let c = {
                let mut c = vec![0; sorted_block.len()];
                let mut idx = 0;
                for &k in sorted_block.iter() {
                    while idx < CHUNK_SIZE
                        && chunk[sorted_chunk[idx]] > a[CHUNK_SIZE * (chunk_idx + 1) + k]
                    {
                        idx += 1;
                    }
                    c[k] = idx;
                }
                c
            };
            sorted_block = Self::merge_sort(
                &a[CHUNK_SIZE * (chunk_idx + 1)..],
                &chunk,
                &sorted_block,
                &sorted_chunk,
            );
            let mut res_i = vec![0];
            for (j, aj) in chunk.iter().enumerate() {
                res_i.push(res_i.last().unwrap() + chunk[..j].iter().filter(|al| *al > aj).count());
            }
            for (k, c) in c.iter().enumerate() {
                res_i.push(
                    res_i.last().unwrap() + res.last().unwrap()[k + 1] + c - res.last().unwrap()[k],
                );
            }
            res.push(res_i);
        }
        res.reverse();
        res
    }

    fn merge_sort<S: Copy + PartialOrd>(a: &[S], b: &[S], x: &[usize], y: &[usize]) -> Vec<usize> {
        let mut res = vec![0; x.len() + y.len()];
        let mut x_idx = 0;
        let mut y_idx = 0;
        while x_idx < x.len() && y_idx < y.len() {
            if a[x[x_idx]] >= b[y[y_idx]] {
                res[x_idx + y_idx] = x[x_idx] + CHUNK_SIZE;
                x_idx += 1;
            } else {
                res[x_idx + y_idx] = y[y_idx];
                y_idx += 1;
            }
        }
        if x_idx < x.len() {
            res[x_idx + y_idx..].clone_from_slice(
                &x[x_idx..]
                    .iter()
                    .map(|x| x + CHUNK_SIZE)
                    .collect::<Vec<_>>(),
            );
        } else if y_idx < y.len() {
            res[x_idx + y_idx..].clone_from_slice(&y[y_idx..]);
        }
        res
    }

    fn calc_ends(&self, l: usize, r: usize, ll: usize, rr: usize) -> usize {
        if r == 0 {
            return 0;
        }
        let mut res = 0;
        let mut l_idx = 0;
        let mut cnt = 0;
        for &(r_idx, r_val) in self.sorted_chunks[rr - 1].iter() {
            while l_idx < CHUNK_SIZE && self.sorted_chunks[ll][l_idx].1 > r_val {
                if l > self.sorted_chunks[ll][l_idx].0 {
                    cnt += 1;
                }
                l_idx += 1;
            }
            if r_idx >= r {
                res += cnt;
            }
        }
        res
    }
}

fn unzip<R: RangeBounds<usize>>(range: R, n: usize) -> (usize, usize) {
    let start = match range.start_bound() {
        Bound::Unbounded => 0,
        Bound::Included(&x) => x,
        Bound::Excluded(&x) => x + 1,
    };
    let end = match range.end_bound() {
        Bound::Unbounded => n,
        Bound::Included(&x) => x + 1,
        Bound::Excluded(&x) => x,
    };
    (start, end)
}
