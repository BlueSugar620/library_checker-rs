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
            let x: (u64, u64, u64, u64) = (
                stdin.next().unwrap().parse().unwrap(),
                stdin.next().unwrap().parse().unwrap(),
                stdin.next().unwrap().parse().unwrap(),
                stdin.next().unwrap().parse().unwrap(),
            );
            let ans = potential_dsu.unite(v, u, x);
            writeln!(stdout, "{}", ans as u8).ok();
        } else {
            let ans = potential_dsu.poteintial(v, u);
            if let Some((a, b, c, d)) = ans {
                writeln!(stdout, "{} {} {} {}", a, b, c, d).ok();
            } else {
                writeln!(stdout, "-1").ok();
            }
        }
    }
}

enum O {}
impl Group for O {
    type Value = (u64, u64, u64, u64);
    fn id() -> Self::Value {
        (1, 0, 0, 1)
    }
    fn mul(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value {
        let a = lhs.0 * rhs.0 % MOD + lhs.1 * rhs.2 % MOD;
        let b = lhs.0 * rhs.1 % MOD + lhs.1 * rhs.3 % MOD;
        let c = lhs.2 * rhs.0 % MOD + lhs.3 * rhs.2 % MOD;
        let d = lhs.2 * rhs.1 % MOD + lhs.3 * rhs.3 % MOD;
        (
            if a >= MOD { a - MOD } else { a },
            if b >= MOD { b - MOD } else { b },
            if c >= MOD { c - MOD } else { c },
            if d >= MOD { d - MOD } else { d },
        )
    }
    fn inv(val: &Self::Value) -> Self::Value {
        (
            val.3,
            if val.1 == 0 { 0 } else { MOD - val.1 },
            if val.2 == 0 { 0 } else { MOD - val.2 },
            val.0,
        )
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
