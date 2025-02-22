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

    let inclusive_scan = InclusiveScan::<O>::new(&a);

    for _ in 0..q {
        let l: usize = stdin.next().unwrap().parse().unwrap();
        let r: usize = stdin.next().unwrap().parse().unwrap();
        writeln!(stdout, "{}", inclusive_scan.fold(l..r)).ok();
    }
}

enum O {}
impl Group for O {
    type Value = u64;
    fn id() -> Self::Value {
        0
    }
    fn mul(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value {
        lhs + rhs
    }
    fn inv(val: &Self::Value) -> Self::Value {
        !val + 1
    }
}

use std::ops::{Bound, RangeBounds};

pub trait Group {
    type Value: Clone;
    fn id() -> Self::Value;
    fn mul(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value;
    fn inv(val: &Self::Value) -> Self::Value;
}

pub struct InclusiveScan<T: Group> {
    scan: Vec<T::Value>,
}

impl<T: Group> InclusiveScan<T> {
    pub fn new(a: &[T::Value]) -> Self {
        Self {
            scan: std::iter::once(T::id())
                .chain(a.to_vec())
                .scan(T::id(), |acc, a| {
                    *acc = T::mul(acc, &a);
                    Some(acc.clone())
                })
                .collect::<Vec<_>>(),
        }
    }

    pub fn fold<R: RangeBounds<usize>>(&self, range: R) -> T::Value {
        let (l, r) = unzip(range, self.scan.len() - 1);
        T::mul(&T::inv(&self.scan[l]), &self.scan[r])
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
