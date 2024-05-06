pub struct Rng;

impl Rng {
    pub fn generate_key(length: usize) -> String {
        let mut rng = Rng {};
        let ascii = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

        (0..length)
            .map(|_| {
                let idx = rand_core::RngCore::next_u64(&mut rng) as usize % ascii.len();
                ascii.chars().nth(idx).unwrap()
            })
            .collect()
    }
}

impl rand_core::RngCore for Rng {
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    fn next_u64(&mut self) -> u64 {
        let mut w = [0u8; 8];
        fill_8_bytes(&mut w);
        u64::from_be_bytes(w)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        for chunk in dest.chunks_mut(8) {
            let mut w = [0u8; 8];
            fill_8_bytes(&mut w);
            chunk.copy_from_slice(&w);
        }
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

fn fill_8_bytes(w: &mut [u8; 8]) {
    let val = ft_sdk::env::random().to_be_bytes();

    for (i, byte) in val.iter().enumerate() {
        w[i] = *byte;
    }
}

impl rand_core::CryptoRng for Rng {}
