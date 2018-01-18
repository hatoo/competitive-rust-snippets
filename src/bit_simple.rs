#[snippet = "BIT-usize"]
#[allow(dead_code)]
struct BIT {
    buf: Vec<usize>,
}

#[snippet = "BIT-usize"]
#[allow(dead_code)]
impl BIT {
    fn new(n: usize) -> BIT {
        BIT {
            buf: vec![0; n + 1],
        }
    }

    fn sum(&self, i: usize) -> usize {
        let mut i = i;
        let mut s = 0;
        while i > 0 {
            s += self.buf[i];
            i &= i - 1;
        }
        s
    }

    fn add(&mut self, i: usize, x: usize) {
        let mut i = i as i64;
        while i < self.buf.len() as i64 {
            self.buf[i as usize] += x;
            i += i & -i;
        }
    }
}

#[test]
fn test_bit_simple_vs_cumsum() {
    use rand::{Rng, SeedableRng, StdRng};
    let size = 1000;
    let mut cum_sum = vec![0; size + 1];
    let mut bit = BIT::new(size);

    let mut rng = StdRng::from_seed(&[1, 2, 3]);

    let mut sum = 0;
    for i in 1..size + 1 {
        let x = rng.next_u32() as usize / (2 * size);
        sum += x;
        cum_sum[i] = sum;
        bit.add(i, x);
    }

    for _ in 0..1000 {
        let i = rng.next_u32() as usize % size + 1;

        assert_eq!(bit.sum(i), cum_sum[i]);
    }
}
