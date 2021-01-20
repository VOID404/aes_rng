use libc::c_void;
use rand_core::{CryptoRng, RngCore, SeedableRng};

mod ffi {
    use libc::c_void;
    extern "C" {
        // void delete_aes_rng(void *ptr)
        pub fn delete_aes_rng(ptr: *mut c_void);

        // AES_RNG* new_aes_rng(const uint8_t *seed_ptr, size_t seed_len)
        pub fn new_aes_rng(seed_ptr: *const u8, seed_len: usize) -> *mut c_void;

        // void fill_buffer(AES_RNG *prng, unsigned char *buf_ptr, size_t buf_len)
        pub fn fill_buffer(prng: *mut c_void, buf_ptr: *mut u8, buf_len: usize);
    }
}

#[derive(Debug)]
pub struct AesRng {
    inner: *mut c_void,
}

// SeedableRng<Seed = [u8; 32]> + RngCore + CryptoRng

use rand_core::impls::{next_u32_via_fill, next_u64_via_fill};

impl RngCore for AesRng {
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        let buf_ptr = dest.as_mut_ptr();
        let buf_len = dest.len();
        unsafe { ffi::fill_buffer(self.inner, buf_ptr, buf_len) }
    }

    fn next_u32(&mut self) -> u32 {
        next_u32_via_fill(self)
    }
    fn next_u64(&mut self) -> u64 {
        next_u64_via_fill(self)
    }
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        Ok(self.fill_bytes(dest))
    }
}

impl CryptoRng for AesRng {}

impl SeedableRng for AesRng {
    type Seed = [u8; 32];

    fn from_seed(seed: Self::Seed) -> Self {
        let seed_ptr = seed.as_ptr();
        let seed_len = seed.len();
        let inner = unsafe { ffi::new_aes_rng(seed_ptr, seed_len) };
        AesRng { inner }
    }
}

impl Drop for AesRng {
    fn drop(&mut self) {
        unsafe { ffi::delete_aes_rng(self.inner) };
    }
}

#[cfg(test)]
mod tests {
    use crate::AesRng;
    use rand_core::{RngCore, SeedableRng};

    #[test]
    fn aes_rng_works() {
        let seed = [0; 32];
        let results: [[u8; 16]; 2] = [
            [
                57, 206, 245, 70, 44, 114, 49, 86, 174, 179, 122, 140, 137, 47, 165, 35,
            ],
            [
                241, 65, 83, 99, 27, 93, 20, 134, 147, 146, 22, 162, 22, 31, 89, 102,
            ],
        ];

        let mut aes = AesRng::from_seed(seed);

        for r in results.iter() {
            let mut buf = vec![0; 16];
            aes.fill_bytes(&mut buf);
            r.into_iter().zip(buf).for_each(|(a, b)| assert_eq!(*a, b));
        }
    }
}
