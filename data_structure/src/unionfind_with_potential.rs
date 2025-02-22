use std::io::Write;

const MOD: u64 = 998_244_353;

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut stdin = stdin.split_whitespace();
    let mut stdout = std::io::BufWriter::new(std::io::stdout().lock());

    let n: usize = stdin.next().unwrap().parse().unwrap();
    let q: usize = stdin.next().unwrap().parse().unwrap();

    let mut potential_dsu = PotentialDSU::<O>::new(n);

    for _ in 0..q {
        let t: u8 = stdin.next().unwrap().parse().unwrap();
        let u: usize = stdin.next().unwrap().parse().unwrap();
        let v: usize = stdin.next().unwrap().parse().unwrap();
        if t == 0 {
            let x: u64 = stdin.next().unwrap().parse().unwrap();
            let ans = potential_dsu.unite(v, u, x);
            writeln!(stdout, "{}", ans as u8).ok();
        } else {
            let ans = potential_dsu.poteintial(v, u);
            if let Some(ans) = ans {
                writeln!(stdout, "{}", ans).ok();
            } else {
                writeln!(stdout, "-1").ok();
            }
        }
    }
}

enum O {}
impl Group for O {
    type Value = u64;
    fn id() -> Self::Value {
        0
    }
    fn mul(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value {
        let res = lhs + rhs;
        if res >= MOD {
            res - MOD
        } else {
            res
        }
    }
    fn inv(val: &Self::Value) -> Self::Value {
        if *val == 0 {
            0
        } else {
            MOD - val
        }
    }
}

pub trait Group {
    type Value: Clone + PartialEq;
    fn id() -> Self::Value;
    fn mul(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value;
    fn inv(val: &Self::Value) -> Self::Value;
}

pub struct PotentialDSU<T: Group> {
    parents: Vec<isize>,
    potentials: Vec<T::Value>,
    cnt: usize,
}

impl<T: Group> PotentialDSU<T> {
    pub fn new(n: usize) -> Self {
        Self {
            parents: vec![-1; n],
            potentials: vec![T::id(); n],
            cnt: n,
        }
    }

    pub fn root(&self, mut v: usize) -> (usize, T::Value) {
        let mut potential = self.potentials[v].clone();
        while self.parents[v] >= 0 {
            v = self.parents[v] as usize;
            potential = T::mul(&self.potentials[v], &potential);
        }
        (v, potential)
    }

    pub fn unite(&mut self, from: usize, to: usize, d: T::Value) -> bool {
        let (mut from, p_from) = self.root(from);
        let (mut to, p_to) = self.root(to);
        if from == to {
            T::mul(&p_from, &d) == p_to
        } else {
            let mut d = T::mul(&T::mul(&p_from, &d), &T::inv(&p_to));
            if self.parents[from] > self.parents[to] {
                std::mem::swap(&mut from, &mut to);
                d = T::inv(&d);
            }
            self.parents[from] += self.parents[to];
            self.parents[to] = from as isize;
            self.potentials[to] = d;
            self.cnt -= 1;
            true
        }
    }

    pub fn poteintial(&self, from: usize, to: usize) -> Option<T::Value> {
        let (from, p_from) = self.root(from);
        let (to, p_to) = self.root(to);
        if from == to {
            Some(T::mul(&T::inv(&p_from), &p_to))
        } else {
            None
        }
    }

    pub fn size(&self, u: usize) -> usize {
        -self.parents[self.root(u).0] as usize
    }

    pub fn cnt(&self) -> usize {
        self.cnt
    }
}
