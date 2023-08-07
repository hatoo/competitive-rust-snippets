#[repr(transparent)]
pub struct PCG32si {
    state: u32,
}

impl PCG32si {
    const PCG_DEFAULT_MULTIPLIER_32: u32 = 747796405;
    const PCG_DEFAULT_INCREMENT_32: u32 = 2891336453;

    fn pcg_oneseq_32_step_r(&mut self) {
        self.state = self
            .state
            .wrapping_mul(Self::PCG_DEFAULT_MULTIPLIER_32)
            .wrapping_add(Self::PCG_DEFAULT_INCREMENT_32);
    }

    fn pcg_output_rxs_m_xs_32_32(state: u32) -> u32 {
        let word = ((state >> ((state >> 28).wrapping_add(4))) ^ state).wrapping_mul(277803737);
        (word >> 22) ^ word
    }

    pub fn new(seed: u32) -> Self {
        let mut rng = Self { state: seed };
        rng.pcg_oneseq_32_step_r();
        rng.state = rng.state.wrapping_add(seed);
        rng.pcg_oneseq_32_step_r();
        rng
    }

    pub fn next_u32(&mut self) -> u32 {
        let old_state = self.state;
        self.pcg_oneseq_32_step_r();
        Self::pcg_output_rxs_m_xs_32_32(old_state)
    }

    pub fn next_f32(&mut self) -> f32 {
        const FLOAT_SIZE: u32 = core::mem::size_of::<f32>() as u32 * 8;
        const PRECISION: u32 = 23 + 1;
        const SCALE: f32 = 1.0 / (1 << PRECISION) as f32;
        const SHIFT: u32 = FLOAT_SIZE - PRECISION;

        let value = self.next_u32();
        let value = value >> SHIFT;
        SCALE * value as f32
    }

    pub fn next_f32_range(&mut self, min: f32, max: f32) -> f32 {
        min + (max - min) * self.next_f32()
    }
}

#[test]
fn test_pcg32si_next_f32() {
    let mut rng = PCG32si::new(0);
    for _ in 0..1_000_000 {
        let f = rng.next_f32();
        assert!(f >= 0.0);
        assert!(f <= 1.0);
    }
}

#[cfg(test)]
use test::Bencher;

#[bench]
fn bench_pcg32si_next(b: &mut Bencher) {
    let mut rng = PCG32si::new(0);
    b.iter(|| {
        for _ in 0..1_000_000 {
            rng.next_u32();
        }
    });
}
