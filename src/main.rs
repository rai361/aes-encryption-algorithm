// use hex

macro_rules! swap_unsigned {
    ($x:expr, $y:expr) => {
        $x ^= $y;
        $y ^= $x;
        $x ^= $y;
    };
}

const POLY: u8 = 0x1b; // x^8 + x^4 + x^3 + x + 1

fn gf2_8_mul(mut a: u8, mut b: u8) -> u8 {
    let mut result = 0;

    for _ in 0..8 {
        if b & 1 == 1{
            result ^= a;
        }

        if a & 0x80 != 0 {
            a = (a << 1) ^ POLY;
        } else {
            a <<= 1;
        }

        b >>= 1;
    }

    return result;
}

const S_BOX: [u8; 256] = [0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76, 0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0, 0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31,0x15, 0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75, 0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84, 0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c,0x58, 0xcf, 0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8, 0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2, 0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64,0x5d, 0x19, 0x73, 0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb, 0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79, 0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea,0x65, 0x7a, 0xae, 0x08, 0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a, 0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e, 0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87,0xe9, 0xce, 0x55, 0x28, 0xdf, 0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16];

#[inline(always)]
fn sub_bytes(block: &mut [u8; 16]) {
    for idx in 0..16 {
        block[idx] = S_BOX[block[idx] as usize];
    }
}

#[inline(always)]
fn shift_rows(block: &mut [u8; 16]) {
    swap_unsigned!(block[1], block[13]);
    swap_unsigned!(block[1], block[5]);
    swap_unsigned!(block[5], block[9]);

    swap_unsigned!(block[2], block[10]);
    swap_unsigned!(block[6], block[14]);
    
    swap_unsigned!(block[11], block[15]);
    swap_unsigned!(block[7], block[11]);
    swap_unsigned!(block[3], block[7]);
}

const MATRIX_COEFF: [u8; 4] = [0x02, 0x03, 0x01, 0x01]; 

#[inline(always)]
fn gf2_coeff_mul(value: u8, coeff: u8) -> u8 {
    if coeff == 0x01 {
        return value;
    } 

    let mut shifted = value << 1;
    
    if value & 0x80 != 0 {
        shifted ^= POLY;
    }

    if coeff == 0x03 {
        shifted ^= value;
    }

    return shifted;
}

#[inline(always)]
fn mix_columns(block: &mut [u8; 16]) {
    let mut values: [u8; 4];

    for x in 0..4 {
        values = [block[x << 2], block[(x << 2) + 1], block[(x << 2) + 2], block[(x << 2) + 3]];
        for y in 0..4 {
            block[y + (x << 2)] = 
                gf2_coeff_mul(values[0], MATRIX_COEFF[(4 - y) & 0b11]) ^ 
                gf2_coeff_mul(values[1], MATRIX_COEFF[(5 - y) & 0b11]) ^ 
                gf2_coeff_mul(values[2], MATRIX_COEFF[(6 - y) & 0b11]) ^ 
                gf2_coeff_mul(values[3], MATRIX_COEFF[(7 - y) & 0b11]);
        }
    }
}

#[inline(always)]
fn add_round_key(block: &mut [u8; 16], round_key: &[u8], round_key_offset: usize) {
    for idx in 0..16 {
        block[idx] ^= round_key[idx + round_key_offset];
    }
}

const ROUNDS: usize = 11;

fn key_expansion_128(key: &[u8; 16]) -> [u8; ROUNDS * 16] {
    let mut buffer: [u8; ROUNDS * 16] = [0; ROUNDS * 16];
    buffer[0..16].copy_from_slice(key);

    let mut idx = 16;
    let mut pow = 0x01;

    while idx < ROUNDS * 16 {
        if idx & 0xF == 0 {
            buffer[idx]     = S_BOX[buffer[idx - 3] as usize] ^ pow;
            buffer[idx + 1] = S_BOX[buffer[idx - 2] as usize];
            buffer[idx + 2] = S_BOX[buffer[idx - 1] as usize];
            buffer[idx + 3] = S_BOX[buffer[idx - 4] as usize];
            
            if pow & 0x80 != 0 {
                pow = (pow << 1) ^ POLY;
            } else {
                pow <<= 1;
            }
        } else {
            buffer[idx]     = buffer[idx - 4];
            buffer[idx + 1] = buffer[idx - 3];
            buffer[idx + 2] = buffer[idx - 2];
            buffer[idx + 3] = buffer[idx - 1];
        }

        buffer[idx]     ^= buffer[idx - 16];
        buffer[idx + 1] ^= buffer[idx - 15];
        buffer[idx + 2] ^= buffer[idx - 14];
        buffer[idx + 3] ^= buffer[idx - 13];

        idx += 4;
    }

    return buffer;
}

fn aes_encrypt_128(block: &mut [u8; 16], key: &[u8; 16]) {
    let round_key = key_expansion_128(key);

    // Initial round
    add_round_key(block, &round_key, 0);
    
    // 9 main rounds
    let mut idx = 1;
    while idx <= 9 {
        sub_bytes(block);
        shift_rows(block);
        mix_columns(block);
        add_round_key(block, &round_key, idx << 4);

        idx += 1;
    }
    
    // Final round
    sub_bytes(block);
    shift_rows(block);
    add_round_key(block, &round_key, 160);
}

fn print_block(block: &[u8; 16]) {
    for y in 0..4 {
        for x in 0..4 {
            print!("{:02X} ", block[y + (x << 2)]);

        }
        println!("");
    }

    println!("");
}

fn print_block_from_hex(plaintext: &str) {
    for (idx, c) in plaintext.chars().enumerate() {
        print!("{}", c);

        if (idx + 1) % 8 == 0 {
            println!("");
        } else if idx & 1 == 1 {
            print!(" ");
        }
    }

    println!("");
}

fn hex_to_block(plaintext: &str) -> [u8; 16] {
    let mut block: [u8; 16] = [0; 16];

    for (idx, c) in plaintext.chars().enumerate() {
        match c {
            '0'..='9' => {
                block[idx >> 1] |= (u32::from(c) as u8 - 0x30) << ((!idx & 1) << 2);
            },
            'A'..='F' => {
                block[idx >> 1] |= (u32::from(c) as u8 - 0x41 + 0xA) << ((!idx & 1) << 2);
            },
            _ => {}
        }

    }

    return block;
}

fn u128_to_be_bytes(value: u128) -> [u8; 16] {
    return (value as u128).to_be_bytes();
}

fn main() {
    let key = u128_to_be_bytes(0x2b7e151628aed2a6abf7158809cf4f3c);
    let mut block = u128_to_be_bytes(0x4C6F72656D20697073756D20646F6C6F);
    
    println!("Key");
    print_block(&key);    
    
    println!("Plaintext");
    print_block(&block);

    aes_encrypt_128(&mut block, &key);
    
    println!("Encrypted");
    print_block(&block);
}