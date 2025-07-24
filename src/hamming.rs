/// # Hamming Encode
/// 
/// Fungsi ini mengambil 4 bit data dan ditambahkan 3 parity bits yang menjadikannya 7 bit.
/// 
/// # Cara Kerja:
/// 1. Menerima 4 bit data
/// 2. Tiga parity bits (p1, p2, dan p3) dihitung dengan operasi XOR (Exclusive OR). Setiap parity bits, bertanggung jawab
/// untuk memeriksa sekelompok data.
/// 3. Bit-bit tersebut disusun dalam urutan standar Hamming, dimana posisi parity bits merupakan pangkat dua (1,2,4,8...)
/// 4. Fungsi ini akan mengembalikan 7 bit data (4 data asli dan 3 parity bits).
pub fn hamming_encode(data: &[u8; 4]) -> [u8; 7] {
    let d1 = data[0];
    let d2 = data[1];
    let d3 = data[2];
    let d4 = data[3];

    let p1 = d1 ^ d2 ^ d4; 
    let p2 = d1 ^ d3 ^ d4;
    let p3 = d2 ^ d3 ^ d4;

    [p1, p2, d1, p3, d2, d3, d4]
}

/// # Hamming Decode
/// 
/// Fungsi ini mengambil data yang telah di encode untuk memeriksa, memperbaiki, dan mengubah data menjadi data aslinya.
/// 
/// # Cara Kerja
/// 1. Cek ulang paritas dengan operasi XOR. Jika tidak ada kesalahan, maka bit sindrom akan bernilai 0.
/// 2. Bit-bit sindrom ini akan digabungkan dan menghasilkan posisi bit yang salah jika ditemukan. Jika hasilnya 0, maka tidak ada kesalahan.
/// 3. Jika error_pos tidak 0, posisi bit yang salah akan dibalik dengan ^= (0 menjadi 1 dan sebaliknya).
/// 4. Fungsi ini mengembalikan 4 bit data asli dari encoded word sebelumnya.
pub fn hamming_decode(mut code: [u8; 7]) -> ([u8; 4], Option<usize>) {
    let p1 = code[0];
    let p2 = code[1];
    let d1 = code[2];
    let p3 = code[3];
    let d2 = code[4];
    let d3 = code[5];
    let d4 = code[6];

    let c1 = p1 ^ d1 ^ d2 ^ d4;
    let c2 = p2 ^ d1 ^ d3 ^ d4;
    let c3 = p3 ^ d2 ^ d3 ^ d4;

    let error_position = c3 * 4 + c2 * 2 + c1;

    let corrected_position: Option<usize>;

    if error_position != 0 {
        let error_index = (error_position - 1) as usize;
        println!("Kesalahan terdeteksi pada posisi code: {}", error_index);
        println!("(Indeks array: {})", error_index);

        code[error_index] = 1 - code[error_index];
        println!("Bit berhasil diperbaiki! Code setelah koreksi: {:?}", code);
        corrected_position = Some(error_index);
    } else {
        println!("Tidak ada kesalahan yang terdeteksi!");
        corrected_position = None;
    }

    let original_data = [code[2], code[4], code[5], code[6]];

    (original_data, corrected_position)
}