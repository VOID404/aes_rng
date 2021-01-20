#include "AES_RNG.cpp"
#include <cryptopp/secblock.h>
#include <memory>

extern "C"
void* new_aes_rng(const uint8_t *seed_ptr, size_t seed_len) {
	CryptoPP::SecByteBlock s(seed_ptr, seed_len);
	return new AES_RNG(s, s.size());
}

extern "C"
void delete_aes_rng(void *ptr) {
	AES_RNG* aes_rng = (AES_RNG *) ptr;
	delete aes_rng;
}

extern "C"
void fill_buffer(void *ptr, unsigned char *buf_ptr, size_t buf_len) {
	AES_RNG* prng = (AES_RNG *) ptr;
	prng->GenerateBlock(buf_ptr, buf_len);
}