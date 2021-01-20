use cpp::cpp;
use std::mem;

cpp! {{
    #include "AES_RNG.hpp"
    #include <cryptopp/secblock.h>
}}

pub fn fill_buffer(seed: &[u8], buf: &mut [u8]) {
    let seed_ptr = seed.as_ptr();
    let seed_len = seed.len();

    let buf_ptr = buf.as_mut_ptr();
    let buf_len = buf.len();

    cpp!(unsafe [seed_ptr as "const uint8_t *", seed_len as "size_t",
                 buf_ptr as "unsigned char *", buf_len as "size_t"] {
        CryptoPP::SecByteBlock seed(seed_ptr, seed_len);
        AES_RNG prng(seed, seed.size());
        prng.GenerateBlock(buf_ptr, buf_len);
    });
    // new_key(const uint8_t *sptr, size_t slen, uint8_t *optr, size_t olen);
}

#[cfg(test)]
mod tests {
    use crate::fill_buffer;

    #[test]
    fn aes_rng_works() {
        let seed = &[1, 2, 3, 4, 5, 6, 7, 8];
        let mut buf = vec![0; 16];
        fill_buffer(seed, &mut buf);

        println!("{:?}", buf);
    }
}
