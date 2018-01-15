use std;

#[snippet = "BitSet"]
const TRUE: &'static bool = &true;
#[snippet = "BitSet"]
const FALSE: &'static bool = &false;

#[derive(Clone, Debug)]
#[snippet = "BitSet"]
struct BitSet {
    buf: Vec<u64>,
}

#[snippet = "BitSet"]
impl BitSet {
    #[allow(dead_code)]
    fn new(size: usize) -> BitSet {
        BitSet {
            buf: vec![0; (size + 63) / 64],
        }
    }

    #[allow(dead_code)]
    fn set(&mut self, i: usize, b: bool) {
        if b {
            self.buf[i >> 6] |= 1 << (i & 63);
        } else {
            self.buf[i >> 6] &= !(1 << (i & 63));
        }
    }

    #[allow(dead_code)]
    fn count_ones(&self) -> u32 {
        self.buf.iter().map(|x| x.count_ones()).sum()
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
        for i in 0..q {
            self.buf[i] = 0;
        }
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
impl<'a> std::ops::BitAndAssign<&'a Self> for BitSet {
    fn bitand_assign(&mut self, rhs: &'a Self) {
        for (a, b) in self.buf.iter_mut().zip(rhs.buf.iter()) {
            *a &= b;
        }
    }
}

#[snippet = "BitSet"]
impl<'a> std::ops::BitAnd<&'a Self> for BitSet {
    type Output = Self;
    fn bitand(mut self, rhs: &'a Self) -> Self {
        self &= rhs;
        self
    }
}

#[snippet = "BitSet"]
impl<'a> std::ops::BitOrAssign<&'a Self> for BitSet {
    fn bitor_assign(&mut self, rhs: &'a Self) {
        for (a, b) in self.buf.iter_mut().zip(rhs.buf.iter()) {
            *a |= b;
        }
    }
}

#[snippet = "BitSet"]
impl<'a> std::ops::BitOr<&'a Self> for BitSet {
    type Output = Self;
    fn bitor(mut self, rhs: &'a Self) -> Self {
        self |= rhs;
        self
    }
}

#[snippet = "BitSet"]
impl<'a> std::ops::BitXorAssign<&'a Self> for BitSet {
    fn bitxor_assign(&mut self, rhs: &'a Self) {
        for (a, b) in self.buf.iter_mut().zip(rhs.buf.iter()) {
            *a ^= b;
        }
    }
}

#[snippet = "BitSet"]
impl<'a> std::ops::BitXor<&'a Self> for BitSet {
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
        for i in 0..shift {
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

        for i in 0..size - shift {
            v[i] = v[i + shift];
        }
        for i in size - shift..size {
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
}
