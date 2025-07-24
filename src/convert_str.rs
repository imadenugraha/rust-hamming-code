use crate::hamming::{hamming_encode, hamming_decode};

/// # String to Bits
/// 
/// Fungsi ini mengambil string literal dan di convert menjadi bits.
/// 
/// # Cara Kerja:
/// Fungsi ini mengiterasi setiap character dari string. Fungsi ini mengekstrak 8 bit didalamnya, dari bit paling signifikan (kiri) ke bit 
/// paling tidak signifikan (kanan) dan mengumpulkannya menjadi satu vector besar.
/// 
fn string_to_bits(s: &str) -> Vec<u8> {
    s.as_bytes()
        .iter()
        .flat_map(|&byte| {
            (0..8).map(move |i| (byte >> (7 - i)) & 1)
        })
        .collect()
}

/// # Bits to String
/// 
/// Fungsi ini mengubah urutan bit dan mengembalikannya menjadi potongan-potongan 8 bit (chunks(8))
/// 
/// # Cara Kerja
/// Setiap potongan 8 bit kemudian digabungkan menjadi satu
/// byte, yang kemudian menjadi char (karakter) dan ditambahkan ke string hasil.
fn bits_to_string(bits: &[u8]) -> String {
    let mut result = String::new();

    for chunk in bits.chunks(8) {
        if chunk.len() < 8 { break; }
        
        let mut byte = 0u8;
        for (i, bit) in chunk.iter().enumerate() {
            byte |= bit << (7 - i);
        }

        result.push(byte as char);
    }
    result
}

/// # Encode Message
/// 
/// Fungsi ini mengambil string untuk diubah dengan fungsi humming_encode().
/// 
/// # Cara Kerja:
/// 1. Mengubah seluruh string menjadi bit dengan fungsi string_to_bits().
/// 2. Memproses urutan bit tersebut menjadi 4 potongan bit (chunks(4)).
/// 3. Untuk setiap potongan 4 bit, menggunakan hamming_encode() untuk menghasilkan 7 bit yang aman.
/// 4. Seluruh 7 bit itu digabungkan menjadi satu vektor besar.
pub fn encode_message(s: &str) -> Vec<u8> {
    let bits = string_to_bits(s);
    let mut encoded = vec![];

    for chunk in bits.chunks(4) {
        let mut block = [0u8; 4];

        for i in 0..chunk.len() {
            block[i] = chunk[i];
        }

        let hamming = hamming_encode(&block);
        encoded.extend_from_slice(&hamming);
    }

    encoded
}

/// # Decode Message
/// 
/// Fungsi ini mengambil encoded message untuk diubah menjadi data aslinya.
/// 
/// # Cara Kerja:
/// 1. Memproses data yang diterima dalam potongan 7 bit (chunks(7)).
/// 2. Untuk setiap potongan 7 bit, menggunakan hamming_decode(). Panggilan ini otomatis akan mendeteksi dan memperbaiki satu kesalahan bit.
/// 3. Hanya 4 data bit yang diambil, sementara informasi kesalahannya dibiarkan (decoded, _).
/// 4. Semua potongan 4 bit digabungkan.
/// 5. Terakhir, bits_to_string() digunakan untuk merubah seluruh bit menjadi string asli.
pub fn decode_message(encoded: &[u8]) -> String {
    let mut decoded_bits = vec![];

    for chunk in encoded.chunks(7) {
        if chunk.len() < 7 { break; }

        let mut block = [0u8; 7];
        block.copy_from_slice(chunk);

        let (arr, _) = hamming_decode(block);

        decoded_bits.extend_from_slice(&arr);
    }

    bits_to_string(&decoded_bits)
}