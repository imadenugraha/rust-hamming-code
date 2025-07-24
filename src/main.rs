mod convert_str;
mod hamming;
mod noise;

use convert_str::{encode_message, decode_message};

fn main() {
    let word: String = "HALO".to_string();
    println!("---Langkah 1: Encoding---");
    println!("Data Asli {}", word);

    let mut word_encoded = encode_message(&word);
    println!("Setelah Encoding: {:?}", word_encoded);
    println!("======================================");

    println!("---Langkah 2: Simulasi Noisy---");
    noise::noise(&mut word_encoded);
    println!("Code setelah noise (rusak): {:?}", word_encoded);
    println!("======================================");

    println!("---Langkah 3: Decoding dan Koreksi---");
    let decoded_word = decode_message(&word_encoded);

    assert_eq!(word, decoded_word);
    println!("Verifikasi Berhasil! Data asli sama dengan data hasil decode.");
    println!("======================================");
}