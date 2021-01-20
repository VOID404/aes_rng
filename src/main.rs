use aes_rng::fill_buffer;

fn main() {
    let seed = &[1, 2, 3, 4, 5, 6, 7, 8];
    let mut buf = vec![0; 16];
    fill_buffer(seed, &mut buf);

    println!("{:?}", buf);
}
