use std;

#[snippet = "BitSet"]
const TRUE: &'static bool = &true;
#[snippet = "BitSet"]
const FALSE: &'static bool = &false;

#[derive(Clone, Debug)]
#[snippet = "BitSet"]
/// Efficient bool collection
pub struct BitSet {
    buf: Vec<u64>,
    size: usize,
}

#[snippet = "BitSet"]
impl BitSet {
    #[allow(dead_code)]
    pub fn new(size: usize) -> BitSet {
        BitSet {
            buf: vec![0; (size + 63) / 64],
            size: size,
        }
    }

    #[allow(dead_code)]
    pub fn set(&mut self, i: usize, b: bool) {
        assert!(i < self.size);
        if b {
            self.buf[i >> 6] |= 1 << (i & 63);
        } else {
            self.buf[i >> 6] &= !(1 << (i & 63));
        }
    }

    #[allow(dead_code)]
    pub fn count_ones(&self) -> u32 {
        self.buf.iter().map(|x| x.count_ones()).sum()
    }

    #[allow(dead_code)]
    fn chomp(&mut self) {
        let r = self.size & 63;
        if r != 0 {
            if let Some(x) = self.buf.last_mut() {
                let d = 64 - r;
                *x = (*x << d) >> d;
            }
        }
    }
}

#[snippet = "BitSet"]
impl std::ops::Index<usize> for BitSet {
    type Output = bool;
    fn index(&self, index: usize) -> &bool {
        [FALSE, TRUE][(self.buf[index >> 6] >> (index & 63)) as usize & 1]
    }
}

#[snippet = "BitSet"]
impl std::ops::ShlAssign<usize> for BitSet {
    fn shl_assign(&mut self, x: usize) {
        let q = x >> 6;
        let r = x & 63;

        if q >= self.buf.len() {
            for x in &mut self.buf {
                *x = 0;
            }
            return;
        }

        if r == 0 {
            for i in (q..self.buf.len()).rev() {
                self.buf[i] = self.buf[i - q];
            }
        } else {
            for i in (q + 1..self.buf.len()).rev() {
                self.buf[i] = (self.buf[i - q] << r) | (self.buf[i - q - 1] >> (64 - r));
            }
            self.buf[q] = self.buf[0] << r;
        }

        for x in &mut self.buf[..q] {
            *x = 0;
        }

        self.chomp();
    }
}

#[snippet = "BitSet"]
impl std::ops::Shl<usize> for BitSet {
    type Output = Self;

    fn shl(mut self, x: usize) -> Self {
        self <<= x;
        self
    }
}

#[snippet = "BitSet"]
impl std::ops::ShrAssign<usize> for BitSet {
    fn shr_assign(&mut self, x: usize) {
        let q = x >> 6;
        let r = x & 63;

        if q >= self.buf.len() {
            for x in &mut self.buf {
                *x = 0;
            }
            return;
        }

        if r == 0 {
            for i in 0..self.buf.len() - q {
                self.buf[i] = self.buf[i + q];
            }
        } else {
            for i in 0..self.buf.len() - q - 1 {
                self.buf[i] = (self.buf[i + q] >> r) | (self.buf[i + q + 1] << (64 - r));
            }
            let len = self.buf.len();
            self.buf[len - q - 1] = self.buf[len - 1] >> r;
        }
        for i in self.buf.len() - q..self.buf.len() {
            self.buf[i] = 0;
        }
    }
}

#[snippet = "BitSet"]
impl std::ops::Shr<usize> for BitSet {
    type Output = Self;

    fn shr(mut self, x: usize) -> Self {
        self >>= x;
        self
    }
}

#[snippet = "BitSet"]
impl<'a> std::ops::BitAndAssign<&'a BitSet> for BitSet {
    fn bitand_assign(&mut self, rhs: &'a Self) {
        for (a, b) in self.buf.iter_mut().zip(rhs.buf.iter()) {
            *a &= *b;
        }
    }
}

#[snippet = "BitSet"]
impl<'a> std::ops::BitAnd<&'a BitSet> for BitSet {
    type Output = Self;
    fn bitand(mut self, rhs: &'a Self) -> Self {
        self &= rhs;
        self
    }
}

#[snippet = "BitSet"]
impl<'a> std::ops::BitOrAssign<&'a BitSet> for BitSet {
    fn bitor_assign(&mut self, rhs: &'a Self) {
        for (a, b) in self.buf.iter_mut().zip(rhs.buf.iter()) {
            *a |= *b;
        }
        self.chomp();
    }
}

#[snippet = "BitSet"]
impl<'a> std::ops::BitOr<&'a BitSet> for BitSet {
    type Output = Self;
    fn bitor(mut self, rhs: &'a Self) -> Self {
        self |= rhs;
        self
    }
}

#[snippet = "BitSet"]
impl<'a> std::ops::BitXorAssign<&'a BitSet> for BitSet {
    fn bitxor_assign(&mut self, rhs: &'a Self) {
        for (a, b) in self.buf.iter_mut().zip(rhs.buf.iter()) {
            *a ^= *b;
        }
        self.chomp();
    }
}

#[snippet = "BitSet"]
impl<'a> std::ops::BitXor<&'a BitSet> for BitSet {
    type Output = Self;
    fn bitxor(mut self, rhs: &'a Self) -> Self {
        self ^= rhs;
        self
    }
}

#[test]
fn test_bitset_set_read() {
    let size = 6400;
    use rand::{Rng, SeedableRng, StdRng};
    let mut set = BitSet::new(size);
    let mut v = vec![false; size];
    let mut rng = StdRng::from_seed(&[1, 2, 3]);

    for i in 0..size {
        let b = rng.next_u32() % 2 == 0;
        set.set(i, b);
        v[i] = b;
    }

    for i in 0..size {
        assert_eq!(set[i], v[i]);
    }
}

#[test]
fn test_bitset_shl() {
    let do_test = |size, shift| {
        use rand::{Rng, SeedableRng, StdRng};
        let mut set = BitSet::new(size);
        let mut v = vec![false; size];
        let mut rng = StdRng::from_seed(&[1, 2, 3]);

        for i in 0..size {
            let b = rng.next_u32() % 2 == 0;
            set.set(i, b);
            v[i] = b;
        }
        for i in (shift..v.len()).rev() {
            v[i] = v[i - shift];
        }
        for i in 0..std::cmp::min(size, shift) {
            v[i] = false;
        }

        set <<= shift;
        for i in 0..size {
            assert_eq!(set[i], v[i]);
        }
    };

    do_test(6400, 640);
    do_test(6400, 114);
    do_test(6400, 514);
    do_test(6400, 6400);
    do_test(6400, 16400);

    let mut t = BitSet::new(310);

    for i in 0..31000 {
        t <<= i;
    }
}

#[test]
fn test_bitset_shr() {
    let do_test = |size, shift| {
        use rand::{Rng, SeedableRng, StdRng};
        let mut set = BitSet::new(size);
        let mut v = vec![false; size];
        let mut rng = StdRng::from_seed(&[1, 2, 3]);

        for i in 0..size {
            let b = rng.next_u32() % 2 == 0;
            set.set(i, b);
            v[i] = b;
        }

        let s = if size >= shift { size - shift } else { 0 };

        for i in 0..s {
            v[i] = v[i + shift];
        }

        for i in s..size {
            v[i] = false;
        }

        set >>= shift;
        for i in 0..size {
            assert_eq!(set[i], v[i]);
        }
    };

    do_test(6400, 640);
    do_test(6400, 114);
    do_test(6400, 514);
    do_test(63, 65);
    do_test(6400, 6400);
    do_test(6400, 16400);

    let mut t = BitSet::new(310);

    for i in 0..31000 {
        t >>= i;
    }
}

#[test]
fn test_bitset_chomp() {
    let mut set1 = BitSet::new(4);
    let mut set2 = BitSet::new(8);

    for i in 0..4 {
        set1.set(i, true);
        set2.set(i, true);
    }

    for i in 4..8 {
        set2.set(i, true);
    }

    set1 <<= 2;
    assert_eq!(set1.count_ones(), 2);
    assert_eq!((set1.clone() | &set2).count_ones(), 4);
    assert_eq!((set1.clone() & &set2).count_ones(), 2);
    assert_eq!((set1.clone() ^ &set2).count_ones(), 2);
}

#[cfg(test)]
use test::Bencher;

#[bench]
fn bench_bitset_dp(b: &mut Bencher) {
    use rand::{Rng, SeedableRng, StdRng};
    let size = 1000;
    let mut v = Vec::new();
    let mut rng = StdRng::from_seed(&[3, 2, 1]);

    for _ in 0..size {
        v.push(rng.next_u32() as usize % size);
    }

    let sum = v.iter().sum::<usize>();
    b.iter(|| {
        let mut bset = BitSet::new(sum + 1);
        bset.set(0, true);

        for &x in &v {
            bset |= &(bset.clone() << x);
        }
    });
}
