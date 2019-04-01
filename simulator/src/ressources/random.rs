pub use rand::SeedableRng;
use uuid::Uuid;

const N: usize = 32;
pub struct RandomSeed(pub [u8; N]);
pub struct Random {
    seed: RandomSeed
}

impl Random {
    pub fn from_uuid(uuid: &Uuid) -> Self {
        let mut uuid_str = uuid.to_string();
        uuid_str = uuid_str.replace("-", "");
        let mut bytes: [u8; N] = [0; N];
        bytes.copy_from_slice(uuid_str.as_bytes());
        Self::from_seed(RandomSeed(bytes.clone()))
    }
}

impl Default for RandomSeed {
    fn default() -> RandomSeed {
        RandomSeed([0; N])
    }
}

impl AsMut<[u8]> for RandomSeed {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl SeedableRng for Random {
    type Seed = RandomSeed;

    fn from_seed(seed: RandomSeed) -> Random {
        Random {
            seed
        }
    }
}