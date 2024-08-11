use rand::Rng;
pub fn generate_random_array(length: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..length).map(|_| rng.gen_range(0..25)).collect()
}