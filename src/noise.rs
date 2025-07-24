use rand::Rng;

/// # Noise
/// 
/// Fungsi ini digunakan untuk melakukan flip 1 bit random dalam code.
/// 
/// # Cara Kerja
/// 1. Melakukan generate index random
/// 2. Melakukan flip bit sesuai index yang telah digenerate
/// 
pub fn noise(codeword: &mut Vec<u8>) {
    let mut rng = rand::rng();

    let error_position = rng.random_range(0..7);

    codeword[error_position] = 1 - codeword[error_position];
}
