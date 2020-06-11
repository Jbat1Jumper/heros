use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
pub use rand::Rng;
use rand::{Error, RngCore, SeedableRng};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SRng {
    seed: u64,
}

impl SRng {
    pub fn new(seed: u64) -> SRng {
        SRng { seed }
    }

    pub fn fork(&mut self) -> SRng {
        SRng {
            seed: self.yield_rng().gen(),
        }
    }

    fn yield_rng(&mut self) -> SmallRng {
        let mut rng = SmallRng::seed_from_u64(self.seed);
        self.seed = rng.gen();
        rng
    }

    pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        slice.shuffle(self);
    }
}

impl RngCore for SRng {
    fn next_u32(&mut self) -> u32 {
        self.yield_rng().next_u32()
    }
    fn next_u64(&mut self) -> u64 {
        self.yield_rng().next_u64()
    }
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.yield_rng().fill_bytes(dest);
    }
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        self.yield_rng().try_fill_bytes(dest)
    }
}
